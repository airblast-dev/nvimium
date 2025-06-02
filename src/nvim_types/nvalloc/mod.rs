// enables error recovery for allocating functions
#[cfg(not(any(miri, test)))]
mod nvdefs;

use std::{ffi::c_void, ptr::NonNull};

use libc::{c_char, malloc, size_t};

#[cfg(not(any(miri, test)))]
pub(crate) use nvdefs::{E_OUTOFMEM, preserve_exit, try_to_free_memory};
use panics::alloc_failed;

#[cfg(not(any(miri, test)))]
use thread_lock::can_call;

#[inline]
pub(crate) fn real_size(size: size_t, count: size_t) -> size_t {
    size.checked_mul(count).unwrap()
}

pub unsafe fn xmalloc(size: size_t, count: size_t) -> NonNull<c_void> {
    let real_size = real_size(size, count);

    if real_size == 0 {
        #[cold]
        #[inline(never)]
        fn cold() {}
        cold();
        return NonNull::dangling();
    }
    #[allow(unused_mut)]
    let mut ptr = unsafe { malloc(real_size) };
    #[cfg(not(any(miri, test)))]
    if ptr.is_null() && can_call() {
        unsafe {
            try_to_free_memory();
            ptr = malloc(real_size);
        }
    }

    if ptr.is_null() {
        alloc_failed();
    }

    NonNull::new(ptr).unwrap()
}

pub unsafe fn xrealloc(
    ptr: *mut c_void,
    size: size_t,
    old_cap: size_t,
    new_cap: size_t,
) -> NonNull<c_void> {
    let real_size = real_size(size, new_cap);

    if real_size == 0 {
        if old_cap != 0 {
            unsafe {
                libc::free(ptr);
            }
        }
        return NonNull::dangling();
    }

    let ptr = unsafe {
        #[allow(unused_mut)]
        let mut new_ptr = libc::realloc(ptr, real_size);

        // we couldnt allocate memory and execution is yielded to us
        // tell neovim to free up some memory and try allocating again
        #[cfg(not(any(miri, test)))]
        if new_ptr.is_null() && can_call() {
            {
                try_to_free_memory();
                new_ptr = libc::realloc(ptr, real_size);
            }

            // we already tried to recover some memory but failed, preserve all files and whatever else
            // the function does so there is no data loss
            if new_ptr.is_null() {
                preserve_exit(E_OUTOFMEM);
            }
        }

        if new_ptr.is_null() {
            alloc_failed()
        }

        new_ptr
    };

    NonNull::new(ptr).unwrap()
}

pub unsafe fn xfree(ptr: *mut c_void, capacity: size_t) {
    if capacity != 0 {
        unsafe {
            libc::free(ptr);
        }
    }
}

#[inline]
#[must_use]
pub unsafe fn xmemdupz(src: *const c_void, len: usize, size: size_t) -> NonNull<c_void> {
    unsafe {
        let ptr = xmalloc(size, len.checked_add(1).unwrap());
        xmemcpyz(ptr.as_ptr(), src, len, size);
        ptr
    }
}

#[inline]
pub unsafe fn xmemcpyz(dst: *mut c_void, src: *const c_void, len: usize, size: size_t) {
    let byte_len = len.checked_mul(size).unwrap();
    assert!(!dst.is_null() && !src.is_null());
    unsafe {
        libc::memcpy(dst, src, byte_len);
        *dst.byte_add(byte_len).cast::<c_char>() = 0;
    }
}
