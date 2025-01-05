use std::{marker::PhantomData, num::NonZeroUsize, ops::Deref, ptr::NonNull};

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


    /// Returns the current capacity
    #[inline(always)]
    pub fn capacity(&self) -> NonZeroUsize {
        self.capacity
    }

    /// The total length of the string excluding the null byte
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
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

            capacity: unsafe { NonZeroUsize::new_unchecked( cap.saturating_add(1) ) },
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
        let ptr = unsafe { libc::realloc(self.data.as_ptr() as *mut libc::c_void, self.len + 1) };
        if ptr.is_null() {
            alloc_failed();
        }
        self.data = unsafe { NonNull::new_unchecked(ptr as *mut libc::c_char) };
        self.capacity = new_capacity;
    }

    /// Create a read only copy of the [`String`]
    ///
    /// Prefer this over cloning the value.
    fn as_thinstr(&self) -> ThinString {
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
}

// If modifying lifetimes of ThinString or related methods, make sure this doesnt compile
//fn borrow_check() {
//    let s = String::new();
//    let th = s.as_thinstr();
//    drop(s);
//    dbg!(th);
//}

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
#[derive(Clone, Copy, Debug)]
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
    /// The lifetime provided must be the same lifetime of the pointer.
    /// See [`String::as_thinstr`] for a function that makes use of this.
    #[inline(always)]
    unsafe fn new<'b>(len: usize, data: NonNull<libc::c_char>) -> ThinString<'a>
    where
        'a: 'b,
    {
        Self {
            len,
            data,
            __p: PhantomData::<&'a u8>,
        }
    }

    /// Returns a slice of the buffers bytes
    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *mut u8, self.len) }
    }

    /// Initialize a [`ThinString`] from raw bytes
    ///
    /// Initializes a [`ThinString`] from a null terminated slice of bytes.
    ///
    /// # Panics
    ///
    /// The bytes must always be terminated with a null byte ("\0") even if empty.
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
