mod nvdefs;

use std::{
    ffi::c_void,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use libc::{c_char, malloc, size_t};
use nvdefs::{E_OUTOFMEM, preserve_exit, try_to_free_memory};
use thread_lock::can_call;

static ALLOC_LOCK: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub fn wait_lock() {
  while ALLOC_LOCK.swap(true, Ordering::SeqCst) {
      cold();
      #[cold]
      #[inline(never)]
      fn cold() {}
      core::hint::spin_loop();
  }
}

#[inline(always)]
pub fn release() {
    ALLOC_LOCK.store(false, Ordering::SeqCst);
}

#[inline]
pub fn real_size(size: size_t, count: size_t) -> size_t {
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
    wait_lock();

    let mut ptr = unsafe { malloc(real_size) };
    if ptr.is_null() && can_call() {
        unsafe {
            try_to_free_memory();
            ptr = malloc(real_size);
        }
    }
    release();
    NonNull::new(ptr).unwrap()
}

pub unsafe fn xrealloc(
    ptr: *mut c_void,
    size: size_t,
    old_cap: size_t,
    new_cap: size_t,
) -> NonNull<c_void> {
    let real_size = real_size(size, new_cap);

    wait_lock();
    if real_size == 0 {
        if old_cap != 0 {
            unsafe {
                libc::free(ptr);
            }
        }
        return NonNull::dangling();
    }

    let ptr = unsafe {
        let mut new_ptr = libc::realloc(ptr, real_size);

        // we couldnt allocate memory and execution is yielded to us
        // tell neovim to free up some memory and try allocating again
        if new_ptr.is_null() && can_call() {
            try_to_free_memory();
            new_ptr = libc::realloc(ptr, real_size);

            // we already tried to recover some memory but failed, preserve all files and whatever else 
            // the function does so there is no data loss
            if new_ptr.is_null() {
                preserve_exit(E_OUTOFMEM);
            }
        }

        new_ptr
    };

    release();

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
