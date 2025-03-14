use core::ptr::NonNull;
use std::ffi::c_void;

use libc::malloc;
use panics::alloc_failed;

#[inline]
#[must_use = "Not using the returned pointer causes a memory leak"]
pub fn xmalloc<T>(count: usize) -> NonNull<T> {
    unsafe { try_xmalloc(count) }.unwrap_or_else(|| alloc_failed())
}

#[inline]
#[must_use = "Not using the returned pointer causes a memory leak"]
pub unsafe fn try_xmalloc<T>(count: usize) -> Option<NonNull<T>> {
    let size = size_of::<T>();
    if size == 0 {
        return Some(NonNull::dangling());
    }

    let new_cap: isize = size.checked_mul(count)?.try_into().ok()?;

    let ptr = unsafe { malloc(new_cap as usize) };
    NonNull::new(ptr as *mut T)
}

#[inline]
#[must_use = "Not using the returned pointer causes a memory leak"]
pub unsafe fn xrealloc<T>(ptr: *mut T, old_cap: usize, cap: usize) -> NonNull<T> {
    unsafe { try_xrealloc(ptr, old_cap, cap).unwrap_or_else(|| alloc_failed()) }
}

#[inline]
#[must_use = "Not using the returned pointer causes a memory leak"]
pub unsafe fn try_xrealloc<T>(ptr: *mut T, old_cap: usize, cap: usize) -> Option<NonNull<T>> {
    if size_of::<T>() == 0 {
        return Some(NonNull::dangling());
    }
    if cap == 0 && old_cap != 0 {
        unsafe { libc::free(ptr as *mut libc::c_void) };
        return Some(NonNull::dangling());
    }
    let ptr = unsafe {
        libc::realloc(
            ptr as *mut libc::c_void,
            size_of::<T>()
                .checked_mul(cap)
                .unwrap_or_else(|| alloc_failed()),
        )
    };
    NonNull::new(ptr as *mut T)
}

#[inline]
pub unsafe fn xfree<T>(ptr: &mut *mut T) {
    unsafe { libc::free(*ptr as *mut libc::c_void) };
    *ptr = core::ptr::null_mut();
}

#[inline]
#[must_use]
pub unsafe fn xmemdupz<T>(src: *const T, len: usize) -> NonNull<T> {
    unsafe {
        let dst: NonNull<T> = xmalloc(len.checked_add(1).unwrap_or_else(|| alloc_failed()));
        xmemcpyz(src, dst.as_ptr(), len);
        dst
    }
}

#[inline]
pub unsafe fn xmemcpyz<T>(src: *const T, dst: *mut T, len: usize) {
    let byte_len = len.checked_mul(size_of::<T>()).unwrap();
    unsafe {
        libc::memcpy(dst as *mut c_void, src as *const c_void, byte_len);
        dst.byte_add(byte_len).cast::<u8>().write(0);
    }
}
