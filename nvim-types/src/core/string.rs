//! The module for neovim String types
//!
//! # Summary
//! Three types of strings exist to allow expressing various borrow checking rules.
//! For more in depth documentation, see their respective documentation page. The text bellow is to
//! just give a general understanding of the types.
//!
//! # [`String`]
//!
//! This type is a growable, mutable, and owned string type.
//! This is the main string that is used when dealing with strings where its length is defined during runtime.
//! The methods implemented for the type are fairly similar to [`std::string::String`] so you are
//! likely to be familiar with them.
//!
//! # [`ThinString`]
//!
//! A reference to an owned [`String`], [`OwnedThinString`] or compile time defined C string.
//! This type can be though of as the &[`str`], or [`CStr`] of the library. Meaning it only lives long as its
//! owner (or forever if initialized at compile time). Mutating a [`ThinString`] is not allowed
//! (excluding unsafe code), and it is not possible to modify its length in any way. A
//! [`ThinString`] is often returned by neovim when calling functions, the exact lifetime of it may
//! differ from function to function.
//!
//! # [`OwnedThinString`]
//!
//! An owned [`ThinString`]. This type can be thought of as a [`Box<str>`] for most intents and
//! purposes. Some strings returned from neovim are owned, but not with a known capacity. This
//! means we are unable to modify its length. However mutating the bytes is now possible (but
//! still unsafe) since the string is still owned.
//!
//! # Functions that take a String
//!
//! You will often not have worry about converting between these types when passing them to a
//! function, as the functions will have trait bounds to convert things where needed. That said,
//! functions may still limit what kind of string can be passed.

use std::{
    borrow::Borrow, ffi::CStr, fmt::Debug, hash::Hash, marker::PhantomData, num::NonZeroUsize,
    ops::Deref, ptr::NonNull,
};

use panics::{alloc_failed, not_null_terminated};

/// A String type passed to wrapper functions
///
/// Compared to [`std`] types, [`String`] is like a null terminated [`Vec<u8>`].
///
/// Neovim does not always check if a null byte is before the end of the string. Some functions
/// work fine with null bytes in the middle of the string others do not. Generally pushing a null
/// byte should be avoided. Pushing a null byte does not cause undefined behavior but
/// rather unspecified behavior (most commonly a string gets cut off once it encounters the first
/// null byte). This does not cause any issues in this library but rather when its passed to
/// neovim.
///
/// # Implementation Details
///
/// If you are only interacting with functions defined in this library you can safely skip this
/// section. These are only important if you are calling FFI functions directly.
///
/// This struct not exactly the same as the String type in neovim, that would be [`ThinString`].
///
/// This is due to a few reasons:
/// - The layout does not allow us to specify the capacity in it fields, this causes issues as it
///     increases the size of data passed to neovim.
/// - The passed string is not always freed by neovim which means we cannot implement [`Drop`] on
///     the string that is passed. Instead, the [`ThinString`] is given to neovim where it does not
///     require ownership. This is done to avoid any double free's or memory leaks.
/// - Since we would be unable to store the capacity in the value provided to neovim, every length
///     change would be a visit to the allocator. Using a [`ThinString`] we are able to store the
///     capacity and avoid many visits to the allocator.
///
/// This means you should provide a [`ThinString`] when calling C bindings directly.
#[repr(C)]
#[derive(Eq)]
pub struct String {
    // TODO: check feasability of overallocating some bytes to store capacity in allocation
    // This might allow us to introduce some optimizations in the API.
    data: *mut libc::c_char,
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
    /// The capacity includes the space for the null byte making it always bigger than zero.
    #[inline(always)]
    pub fn capacity(&self) -> NonZeroUsize {
        self.capacity
    }

    /// The total length of the string excluding the null byte
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Set the current length of the [`String`]
    ///
    /// Shrinking and growing the length is Undefined Behavior if the bytes are not initialized.
    /// The length value should not include the null byte.
    ///
    /// # Safety
    ///
    /// The length should only be modified if the null byte is moved to the end of the allocation
    /// and enough space is allocated.
    #[inline(always)]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    /// Get a immutable pointer to the buffer
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.data as *const u8
    }

    /// Get a mutable pointer to the buffer
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data as *mut u8
    }

    /// Get the buffer as a slice
    ///
    /// The slice does not include the null byte. For a slice that does include the null byte use
    /// [`String::as_slice_with_null`].
    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        self.as_thinstr().as_slice()
    }

    /// Get the buffer as a slice with the terminating null byte
    #[inline(always)]
    pub fn as_slice_with_null(&self) -> &[u8] {
        self.as_thinstr().as_slice_with_null()
    }

    /// Allocate a [`String`] with a capacity
    ///
    /// Allocates for cap + 1 to make the [`String`] null terminated.
    pub fn with_capacity(cap: usize) -> Self {
        let ptr = unsafe { libc::malloc(cap + 1) };
        if ptr.is_null() {
            alloc_failed();
        }
        let ptr = ptr as *mut libc::c_char;
        unsafe { ptr.write(0) };
        Self {
            len: 0,
            data: ptr,

            capacity: unsafe { NonZeroUsize::new_unchecked(cap.saturating_add(1)) },
        }
    }

    #[inline(always)]
    fn remaining_capacity(&self) -> usize {
        self.capacity.get() - self.len - 1
    }

    /// Reserve space for additional elements
    ///
    /// Does not allocate if enough space is available.
    /// If allocating this function will allocate extra space to avoid multiple visit to the
    /// allocator, where that is not desired use [`String::reserve_exact`].
    pub fn reserve(&mut self, additional: usize) {
        let Some(min_cap) = self.minimum_alloc_capacity(additional) else {
            return;
        };

        let new_capacity = min_cap.checked_next_power_of_two().unwrap_or(min_cap);
        self.realloc(new_capacity);
    }

    /// Reserve space for additional elements
    ///
    /// Does not allocate if enough space is available. This will allocate the minimum amount of
    /// space possible when allocating.
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
        let ptr = unsafe { libc::realloc(self.data as *mut libc::c_void, new_capacity.get()) };
        if ptr.is_null() {
            alloc_failed();
        }
        self.data = ptr as *mut libc::c_char;
        self.capacity = new_capacity;
    }

    /// Create a read only copy of the [`String`]
    ///
    /// Prefer this over cloning the value. When passing to raw C functions this should be used
    /// instead of [`String`].
    #[inline(always)]
    pub const fn as_thinstr(&self) -> ThinString {
        unsafe { ThinString::new(self.len, self.data) }
    }

    /// Leaks the [`String`]
    ///
    /// To avoid memory leaks the allocation must be dropped manually or its ownership must pass an
    /// FFI boundry where the foreign function will free it. Almost always [`String::as_thinstr`]
    /// should be preferred unless you really know you need this.
    pub(crate) fn leak(self) -> ThinString<'static> {
        let th = unsafe { ThinString::new(self.len, self.data) };
        std::mem::forget(self);
        th
    }

    /// Push some bytes to the end of the [`String`]
    ///
    /// This will allocate the minimal amount needed to add the bytes. When pushing bytes in a loop
    /// prefer [`String`]'s [`Extend`] implementation.
    pub fn push<'a, B: 'a + AsRef<[u8]>>(&mut self, string: B) {
        let slice = string.as_ref();
        self.reserve_exact(slice.len());
        // SAFETY: self.data is NonNull and we have reserved space to push the string
        // it is now safe to copy the bytes
        //
        // preferred to use libc::memcpy for better binary size
        unsafe {
            libc::memcpy(
                self.data.add(self.len()) as *mut libc::c_void,
                slice.as_ptr() as *mut libc::c_void,
                slice.len(),
            );
        }

        // SAFETY: the values have been initialized above, it is now safe to set the new length.
        unsafe { self.set_len(self.len() + slice.len()) };

        // SAFETY: we already had enough space, just write the null byte
        unsafe { self.data.add(self.len()).write(0) };
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let mut s = Self::with_capacity(self.len);
        s.push(self.as_slice());
        s
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe { self.set_len(0) };
        self.reserve_exact(source.len());
        self.push(source.as_slice());
    }
}
impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: AsRef<[u8]>> From<B> for String {
    fn from(value: B) -> Self {
        let s = value.as_ref();
        let mut st = String::with_capacity(s.len());
        st.push(s);
        st
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

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.as_thinstr() == other.as_thinstr()
    }
}

impl<'a> PartialEq<ThinString<'a>> for String {
    fn eq(&self, other: &ThinString<'a>) -> bool {
        other.eq(self)
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        self.as_slice() == other.as_bytes()
    }
}

impl PartialEq<[u8]> for String {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

impl PartialEq<CStr> for String {
    fn eq(&self, other: &CStr) -> bool {
        self.as_thinstr().eq(other)
    }
}

impl<T: AsRef<[u8]>> PartialEq<T> for String {
    fn eq(&self, other: &T) -> bool {
        self.as_slice() == other.as_ref()
    }
}

impl Hash for String {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_slice_with_null());
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
        unsafe { libc::free(self.data as *mut libc::c_void) };
    }
}

impl<'a> From<ThinString<'a>> for String {
    fn from(value: ThinString<'a>) -> Self {
        let mut s = Self::with_capacity(value.len());
        s.push(value.as_slice());
        s
    }
}

#[repr(C)]
#[derive(Clone, Copy, Eq)]
pub struct ThinString<'a> {
    data: *mut libc::c_char,
    len: libc::size_t,
    __p: PhantomData<&'a u8>,
}

/// A non-owned string type
///
/// [`ThinString`] can be constructed by calling [`String::as_thinstr`], or one of its [`TryFrom`]
/// implementations that accept any byte slice that is terminated with a null byte.
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
    const unsafe fn new<'b>(len: usize, data: *mut libc::c_char) -> ThinString<'a>
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
        self.data.cast::<u8>() as *const u8
    }

    /// Returns the length of the string excluding the null byte
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns a slice of the buffers bytes without a null byte
    #[inline(always)]
    pub const fn as_slice(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data as *mut u8, self.len) }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Returns a slice of the buffers bytes without a null byte
    #[inline(always)]
    pub const fn as_slice_with_null(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.data as *mut u8, self.len + 1) }
    }

    /// Initialize a [`ThinString`] from raw bytes
    ///
    /// Initializes a [`ThinString`] from a null terminated slice of bytes.
    ///
    /// # Panics
    ///
    /// The bytes must always be terminated with a null byte (0 or "\0") even if empty.
    #[inline]
    pub const fn from_null_terminated(b: &'a [u8]) -> ThinString<'a> {
        let last = b.last().copied();
        match last {
            Some(1..) | None => not_null_terminated(last),
            _ => {}
        }

        Self {
            len: b.len() - 1,
            // SAFETY: slice pointers are always NonNull to optimize for use in enums.
            data: b.as_ptr() as *mut libc::c_char,
            __p: PhantomData::<&'a u8>,
        }
    }
}

// TODO: use the trycompile crate
// If modifying lifetimes of ThinString or related methods, make sure these doesnt compile
//  fn borrow_check() {
//      let s = String::new();
//      let th = s.as_thinstr();
//      drop(s);
//      dbg!(th);
//  }
//  fn mut_check() {
//      let mut s = String::new();
//      let th = s.as_thinstr();
//      s.reserve_exact(1);
//      dbg!(th);
//  }
//  fn slice_check() {
//      let mut s = String::new();
//      let sl = s.as_thinstr().as_slice();
//      s.reserve_exact(1);
//      dbg!(sl);
//  }

impl PartialEq for ThinString<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data || self.as_slice() == other.as_slice()
    }
}

impl PartialEq<String> for ThinString<'_> {
    fn eq(&self, other: &String) -> bool {
        self.data == other.data || self.as_slice() == other.as_thinstr().as_slice()
    }
}

impl<T: AsRef<[u8]>> PartialEq<T> for ThinString<'_> {
    fn eq(&self, other: &T) -> bool {
        self.as_slice() == other.as_ref()
    }
}

impl PartialEq<[u8]> for ThinString<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

impl PartialEq<CStr> for ThinString<'_> {
    fn eq(&self, other: &CStr) -> bool {
        self.as_slice_with_null() == other.to_bytes_with_nul()
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
            data: c"".as_ptr() as *mut libc::c_char,
            len: 0,
            __p: PhantomData,
        }
    }
}

impl<'a> From<&'a CStr> for ThinString<'a> {
    fn from(value: &'a CStr) -> Self {
        Self {
            len: value.count_bytes(),
            data: value.as_ptr() as *mut libc::c_char,
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
            data: value.as_ptr() as *mut libc::c_char,
            __p: PhantomData,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ThinStringError {
    NotNullTerminated,
    Empty,
}

pub trait AsThinString {
    fn as_thinstring(&self) -> ThinString<'_>;
}

impl AsThinString for String {
    #[inline(always)]
    fn as_thinstring(&self) -> ThinString<'_> {
        self.as_thinstr()
    }
}

impl AsThinString for ThinString<'_> {
    #[inline(always)]
    fn as_thinstring(&self) -> ThinString<'_> {
        *self
    }
}

#[cfg(test)]
mod string_alloc {
    use super::String;

    #[test]
    fn new() {
        let s = String::new();
        assert_eq!(s.capacity().get(), 1);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data }, 0);
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
        assert_eq!(unsafe { *s.data }, 0);

        let s = String::with_capacity(10);
        assert_eq!(s.capacity().get(), 11);
        assert_eq!(unsafe { *s.data }, 0);
    }

    #[test]
    fn reserve() {
        let mut s = String::new();
        s.reserve(2);
        assert_eq!(s.capacity().get(), 4);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data }, 0);

        s.reserve(5);
        assert_eq!(s.capacity().get(), 8);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data }, 0);
    }

    #[test]
    fn reserve_exact() {
        let mut s = String::new();
        assert_eq!(unsafe { *s.data }, 0);

        s.reserve_exact(1);
        assert_eq!(s.capacity().get(), 2);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data }, 0);

        s.reserve_exact(5);
        assert_eq!(s.capacity().get(), 6);
        assert_eq!(s.len(), 0);
        assert_eq!(unsafe { *s.data }, 0);
    }

    #[test]
    fn as_thinstr() {
        let s = String::new();
        let th = s.as_thinstr();
        assert_eq!(unsafe { *th.data }, 0);
        assert_eq!(th.data, s.data);
        assert_eq!(th.len, s.len);
        unsafe { assert_eq!(th.data.as_ref(), s.data.as_ref()) };
    }

    #[test]
    fn push() {
        let mut s = String::new();
        assert_eq!(unsafe { *s.data }, 0);
        s.push(b"abc");
        assert_eq!(s.capacity().get(), 4);
        assert_eq!(s.len(), 3);
        assert_eq!(s.as_thinstr().as_slice(), b"abc");
        assert_eq!(unsafe { *s.data.add(3) }, 0);
        s.push(b"123");
        assert_eq!(s.capacity().get(), 7);
        assert_eq!(s.len(), 6);
        assert_eq!(s.as_thinstr().as_slice(), b"abc123");
        assert_eq!(unsafe { *s.data.add(6) }, 0);
    }

    #[test]
    fn clone() {
        // growing
        let s = String::from("Hello");
        let mut s1 = String::new();
        s1.clone_from(&s);
        assert_eq!(s1.as_slice_with_null(), s.as_slice_with_null());
        assert_eq!(s1.len(), 5);
        assert_eq!(s1.capacity().get(), 6);
        assert_eq!(s1.len() + 1, s1.capacity().get());

        // shrinking
        let s = String::from("hi");
        let mut s1 = String::from("Hello");
        s1.clone_from(&s);
        assert_eq!(s1.as_slice_with_null(), s.as_slice_with_null());
        assert_eq!(s1.len(), 2);
        assert_eq!(s1.capacity().get(), 6);
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
    use super::{String, ThinString, ThinStringError};

    fn new_s() -> String {
        let mut s = String::new();
        s.push(b"aasdas");
        s
    }

    #[test]
    fn default() {
        let th = ThinString::default();
        assert!(th.is_empty());
        assert!(!th.as_ptr().is_null());
        assert!(unsafe { *th.as_ptr() } == 0);
    }

    #[test]
    fn eq() {
        let s = new_s();
        let th = ThinString::default();

        assert_ne!(s, th);
        assert_eq!(s, s.as_thinstr());
        assert_eq!(s.as_thinstr(), s);
        let s2 = new_s();
        assert_eq!(s, s2);
        assert_eq!(s.as_thinstr(), s2.as_thinstr());
        assert_eq!(s.as_thinstr(), s2);
        assert_eq!(s2, s.as_thinstr());

        assert_eq!(s, "aasdas");
        // TODO: add more tests
    }

    #[test]
    fn from() {
        let th = ThinString::from(c"Hello");
        assert_eq!(th, "Hello");
    }

    #[test]
    fn try_from() -> Result<(), ThinStringError> {
        let th = ThinString::try_from("a\0")?;
        assert_eq!(th, "a");
        let th = ThinString::try_from(b"a\0".as_slice())?;
        assert_eq!(th, "a");

        Ok(())
    }

    #[test]
    fn try_from_fails() {
        assert!(ThinString::try_from("a").is_err());
        assert!(ThinString::try_from(b"a".as_slice()).is_err());
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

    #[test]
    fn from_null_terminated() {
        let _th = ThinString::from_null_terminated(c"Hello".to_bytes_with_nul());
    }

    #[test]
    #[should_panic]
    fn from_null_terminated_no_null() {
        let _th = ThinString::from_null_terminated("Hello".as_bytes());
    }
}
