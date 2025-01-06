use std::{
    borrow::Borrow, ffi::CStr, fmt::Debug, marker::PhantomData, num::NonZeroUsize, ops::Deref,
    ptr::NonNull,
};

use panics::{alloc_failed, not_null_terminated};

/// A String type passed to wrapper functions
///
/// This is not exactly the same as the String type in neovim, that would be [`ThinString`].
///
/// This is due to a few reasons:
/// - The layout does not allow us to specify the capacity in it fields, this causes issues as it
///     increases the size of data passed to neovim.
/// - The passed string is not always freed by neovim which means we cannot implement [`Drop`] on
///     the string that is passed. Instead, the [`ThinString`] is given to neovim. This is done to
///     avoid any double free's or memory leaks.
/// - Since we would be unable to store the capacity in the value provided to neovim, every length
///     change would be a visit to the allocator. Using a [`ThinString`] we are able to store the
///     capacity and avoid many visits to the allocator.
///
/// Compared to [`std`] types, [`String`] is like a [`Vec<u8>`] and a [`ThinString`] is like a `&str`.
///
/// To avoid using conversion methods on every call site, [`String`] implements [`std::ops::Deref`] for
/// [`ThinString`].
///
/// This also means you should provide a [`ThinString`] when calling C bindings directly.
#[repr(C)]
struct String {
    // TODO: check feasability of overallocating some bytes to store capacity in allocation
    // This might allow us to introduce some optimizations in the API.
    data: NonNull<libc::c_char>,
    len: libc::size_t,

    // when passing to neovim, the rest of the fields must be trimmed out
    capacity: NonZeroUsize,
}

// all methods that change the size of the buffer, or convert self to another type must be placed
// here.
//
// anything else should be implemented on ThinString
impl String {
    #[inline(always)]
    fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Returns the capacity of the buffer
    ///
    /// The capacity includes the space for the null byte making it always larger than zero.
    #[inline(always)]
    pub fn capacity(&self) -> NonZeroUsize {
        self.capacity
    }

    /// The total length of the string excluding the null byte
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr() as *const u8
    }

    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data.as_ptr() as *mut u8
    }

    /// Allocate a [`String`] with a capacity
    ///
    /// Allocates for cap + 1 to make the [`String`] null terminated.
    fn with_capacity(cap: usize) -> Self {
        let ptr = unsafe { libc::malloc(cap + 1) };
        if ptr.is_null() {
            alloc_failed();
        }
        let ptr = ptr as *mut libc::c_char;
        unsafe { ptr.write(0) };
        Self {
            len: 0,
            data: unsafe { NonNull::new_unchecked(ptr) },

            capacity: unsafe { NonZeroUsize::new_unchecked(cap.saturating_add(1)) },
        }
    }

    #[inline(always)]
    fn remaining_capacity(&self) -> usize {
        self.capacity.get() - self.len - 1
    }

    pub fn reserve(&mut self, additional: usize) {
        let Some(min_cap) = self.minimum_alloc_capacity(additional) else {
            return;
        };

        let new_capacity = min_cap.checked_next_power_of_two().unwrap_or(min_cap);
        self.realloc(new_capacity);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        let Some(new_cap) = self.minimum_alloc_capacity(additional) else {
            return;
        };
        self.realloc(new_cap);
    }

    #[inline(always)]
    fn minimum_alloc_capacity(&self, additional: usize) -> Option<NonZeroUsize> {
        let remaining = self.remaining_capacity();
        if remaining >= additional {
            return None;
        }
        unsafe {
            Some(NonZeroUsize::new_unchecked(
                self.capacity.get() + additional - remaining,
            ))
        }
    }

    fn realloc(&mut self, new_capacity: NonZeroUsize) {
        let ptr =
            unsafe { libc::realloc(self.data.as_ptr() as *mut libc::c_void, new_capacity.get()) };
        if ptr.is_null() {
            alloc_failed();
        }
        self.data = unsafe { NonNull::new_unchecked(ptr as *mut libc::c_char) };
        self.capacity = new_capacity;
    }

    /// Create a read only copy of the [`String`]
    ///
    /// Prefer this over cloning the value.
    #[inline(always)]
    pub const fn as_thinstr(&self) -> ThinString {
        unsafe { ThinString::new(self.len, self.data) }
    }

    /// Leaks the [`String`]
    ///
    /// To avoid memory leaks the allocation must be dropped manually or its ownership must pass an
    /// FFI boundry where the foreign function will free it. Almost always [`String::as_thinstr`]
    /// should be preferred unless you really know you need this.
    fn leak(self) -> ThinString<'static> {
        let th = unsafe { ThinString::new(self.len, self.data) };
        std::mem::forget(self);
        th
    }

    fn push<'a, B: 'a + AsRef<[u8]>>(&mut self, string: B) {
        let slice = string.as_ref();
        self.reserve_exact(slice.len());
        // SAFETY: self.data is NonNull and we have reserved space to push the string
        // it is now safe to copy the bytes
        //
        // preferred to use libc::memcpy for better binary size
        unsafe {
            libc::memcpy(
                self.data.add(self.len()).as_ptr() as *mut libc::c_void,
                slice.as_ptr() as *mut libc::c_void,
                slice.len(),
            );
        }

        // SAFETY: the values have been initialized above, it is now safe to set the new length.
        unsafe { self.set_len(self.len() + slice.len()) };

        // SAFETY: we already had enough space, just write the null byte
        unsafe { self.data.as_ptr().add(self.len()).write(0) };
    }
}

impl<'a> Extend<&'a [u8]> for String {
    fn extend<T: IntoIterator<Item = &'a [u8]>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();
        while let Some(sl) = iter.next() {
            self.reserve(sl.len() + iter.size_hint().0);
            self.push(sl);
        }
    }
}

impl std::io::Write for String {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.push(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.push(buf);
        Ok(())
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        let additional = bufs.iter().map(|s| s.len()).sum();
        self.reserve_exact(additional);
        for buf in bufs {
            self.push(buf.deref());
        }

        Ok(additional)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = std::string::String::from_utf8_lossy(self.as_thinstr().as_slice());
        let mut ds = f.debug_struct("String");

        ds.field("data", &self.data)
            .field("len", &self.len())
            .field("capacity", &self.capacity())
            .field("repr", &l.as_ref());

        ds.finish()
    }
}

const _: () = assert!(
    std::mem::size_of::<usize>() + std::mem::size_of::<ThinString>()
        == std::mem::size_of::<String>()
);

impl Drop for String {
    fn drop(&mut self) {
        unsafe { libc::free(self.data.as_ptr() as *mut libc::c_void) };
    }
}

#[repr(C)]
#[derive(Clone, Copy, Eq)]
struct ThinString<'a> {
    data: NonNull<libc::c_char>,
    len: libc::size_t,
    __p: PhantomData<&'a u8>,
}

impl<'a> ThinString<'a> {
    /// Initialize a new ThinString using a pointer and a length
    ///
    /// # Safety
    ///
    /// The lifetime provided must be the same lifetime of the pointer and `data.add(len)` must
    /// point to a null byte.
    ///
    /// See [`String::as_thinstr`] for a function that makes use of this.
    #[inline(always)]
    const unsafe fn new<'b>(len: usize, data: NonNull<libc::c_char>) -> ThinString<'a>
    where
        'a: 'b,
    {
        Self {
            len,
            data,
            __p: PhantomData::<&'a u8>,
        }
    }

    /// Returns a pointer to the first byte in the buffer
    ///
    /// Derefrencing the pointer is always safe as it is non null and the pointer will always
    /// point to a readable value. If the [`ThinString`] is empty the first byte is always a null byte
    /// (0, b"\0").
    ///
    /// For similar reasons to [`std::ffi::CStr`] this does not allow mutating the buffer. Thus the
    /// returned pointer can be cast to a *mut but it should never be mutated.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.data.cast::<u8>().as_ptr() as *const u8
    }

    /// Returns a slice of the buffers bytes without a null byte
    #[inline(always)]
    pub const fn as_slice(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *mut u8, self.len) }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Returns a slice of the buffers bytes without a null byte
    #[inline(always)]
    pub const fn as_slice_with_null(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *mut u8, self.len + 1) }
    }

    /// Initialize a [`ThinString`] from raw bytes
    ///
    /// Initializes a [`ThinString`] from a null terminated slice of bytes.
    ///
    /// # Panics
    ///
    /// The bytes must always be terminated with a null byte (0 or "\0") even if empty.
    #[inline]
    pub fn from_null_terminated<B: 'a + AsRef<[u8]>>(b: B) -> ThinString<'a> {
        let slice = b.as_ref();
        let last = slice.last().copied();
        if last.is_none_or(|l| l != 0) {
            not_null_terminated(last);
        }

        Self {
            len: slice.len(),
            // SAFETY: slice pointers are always NonNull to optimize for use in enums.
            data: unsafe { NonNull::new_unchecked(slice.as_ptr() as *mut libc::c_char) },
            __p: PhantomData::<&'a u8>,
        }
    }
}

// If modifying lifetimes of ThinString or related methods, make sure these doesnt compile
//fn borrow_check() {
//    let s = String::new();
//    let th = s.as_thinstr();
//    drop(s);
//    dbg!(th);
//}
//fn mut_check() {
//    let mut s = String::new();
//    let th = s.as_thinstr();
//    s.reserve_exact(1);
//    dbg!(th);
//}

impl PartialEq for ThinString<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data || self.as_slice() == other.as_slice()
    }
}

impl Debug for ThinString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = std::string::String::from_utf8_lossy(self.as_slice());
        let mut ds = f.debug_struct("ThinString<'_>");
        ds.field("data", &self.data)
            .field("len", &self.len)
            .field("repr", &l)
            .finish()
    }
}

impl Borrow<[u8]> for ThinString<'_> {
    fn borrow(&self) -> &[u8] {
        self.as_slice_with_null()
    }
}

impl Default for ThinString<'static> {
    fn default() -> Self {
        Self {
            data: unsafe { NonNull::new_unchecked(c"".as_ptr() as *mut libc::c_char) },
            len: 0,
            __p: PhantomData,
        }
    }
}

impl<'a> From<&'a CStr> for ThinString<'a> {
    fn from(value: &'a CStr) -> Self {
        Self {
            len: value.count_bytes(),
            data: unsafe { NonNull::new_unchecked(value.as_ptr() as *mut libc::c_char) },
            __p: PhantomData,
        }
    }
}

impl<'a> TryFrom<&'a str> for ThinString<'a> {
    type Error = ThinStringError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for ThinString<'a> {
    type Error = ThinStringError;
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ThinStringError::Empty);
        }
        if value[value.len() - 1] != 0 {
            return Err(ThinStringError::NotNullTerminated);
        }

        Ok(Self {
            len: value.len() - 1,
            data: unsafe { NonNull::new_unchecked(value.as_ptr() as *mut libc::c_char) },
            __p: PhantomData,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum ThinStringError {
    NotNullTerminated,
    Empty,
}

#[cfg(test)]
mod string_alloc {
    use super::String;

    #[test]
    fn new() {
        let s = String::new();
        assert_eq!(s.capacity().get(), 1);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);
    }

    #[test]
    fn capacity() {
        let mut s = String::new();
        assert_eq!(s.capacity().get(), 1);
        s.reserve_exact(5);
        assert_eq!(s.capacity().get(), 6);
    }

    #[test]
    fn with_capacity() {
        let s = String::with_capacity(0);
        assert_eq!(s.capacity().get(), 1);
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);

        let s = String::with_capacity(10);
        assert_eq!(s.capacity().get(), 11);
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);
    }

    #[test]
    fn reserve() {
        let mut s = String::new();
        s.reserve(2);
        assert_eq!(s.capacity().get(), 4);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);

        s.reserve(5);
        assert_eq!(s.capacity().get(), 8);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);
    }

    #[test]
    fn reserve_exact() {
        let mut s = String::new();
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);

        s.reserve_exact(1);
        assert_eq!(s.capacity().get(), 2);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);

        s.reserve_exact(5);
        assert_eq!(s.capacity().get(), 6);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);
    }

    #[test]
    fn as_thinstr() {
        let s = String::new();
        let th = s.as_thinstr();
        assert_eq!(unsafe { *th.data.as_ptr() }, 0);
        assert_eq!(th.data, s.data);
        assert_eq!(th.len, s.len);
        unsafe { assert_eq!(th.data.as_ref(), s.data.as_ref()) };
    }

    #[test]
    fn push() {
        let mut s = String::new();
        assert_eq!(unsafe { *s.data.as_ptr() }, 0);
        s.push(b"abc");
        assert_eq!(s.capacity().get(), 4);
        assert_eq!(s.len(), 3);
        assert_eq!(s.as_thinstr().as_slice(), b"abc");
        assert_eq!(unsafe { *s.data.as_ptr().add(3) }, 0);
        s.push(b"123");
        assert_eq!(s.capacity().get(), 7);
        assert_eq!(s.len(), 6);
        assert_eq!(s.as_thinstr().as_slice(), b"abc123");
        assert_eq!(unsafe { *s.data.as_ptr().add(6) }, 0);
    }
}

#[cfg(test)]
mod string_fmt {
    use crate::string::{String, ThinString};

    #[test]
    fn debug_string() {
        let f = format!("{:?}", String::new());
        let pre_ptr = &f[0..15];
        let post_ptr = {
            let len_start = f.find("len:").unwrap();
            &f[len_start..]
        };
        assert_eq!(pre_ptr, "String { data: ");
        assert_eq!(post_ptr, "len: 0, capacity: 1, repr: \"\" }");
    }

    #[test]
    fn debug_thinstring() {
        let f = format!("{:?}", ThinString::default());
        let pre_ptr = &f[0..23];
        let post_ptr = {
            let len_start = f.find("len:").unwrap();
            &f[len_start..]
        };
        assert_eq!(pre_ptr, "ThinString<'_> { data: ");
        assert_eq!(post_ptr, "len: 0, repr: \"\" }");
    }
}

#[cfg(test)]
mod thinstr {
    use super::{String, ThinString};

    fn new_s() -> String {
        let mut s = String::new();
        s.push(b"aasdas");
        s
    }

    #[test]
    fn as_slice() {
        let th = ThinString::default();
        assert_eq!(th.as_slice(), &[]);
        assert_eq!(th.as_slice_with_null(), &[0]);

        let s = new_s();
        let slice = s.as_thinstr().as_slice();
        assert_eq!(slice, b"aasdas");

        let slice = s.as_thinstr().as_slice_with_null();
        assert_eq!(slice, b"aasdas\0");
    }

    #[test]
    fn as_ptr() {
        let s = new_s();
        let th = s.as_thinstr();
        assert_eq!(s.as_ptr(), th.as_ptr());
    }

    #[test]
    fn is_empty() {
        let mut s = String::new();
        assert!(s.as_thinstr().is_empty());
        s.push("bawawa");
        assert!(!s.as_thinstr().is_empty());
    }
}
