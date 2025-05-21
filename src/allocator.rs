#[cfg(not(any(miri, test)))]
use crate::nvim_types::nvalloc::try_to_free_memory;
use std::alloc::{GlobalAlloc, System};
#[cfg(not(any(miri, test)))]
use thread_lock::can_call;

/// A wrapper around [`std::alloc::System`] that reclaims extra memory used by Neovim
///
/// Neovim provides a C function we can call in order trigger a garbage collection in Lua and
/// release of the unused memory blocks stored by Neovim (this non-exhaustive but you get the idea).
///
/// Considering most plugins will be single threaded it makes sense to set this allocator as the
/// global allocator in order to recover from memory exhaustion.
///
/// # Note
///
/// Thread safety is achieved by [`thread_lock::can_call`] to ensure it is safe to call Neovim
/// functions. In case we are not allowed to call a Neovim function in the context an allocation
/// was invoked, this allocator will simply return whatever value is returned by its corresponding
/// function from [`std::alloc::System`]. It is also worth noting that this will add an extra [`bool`]
/// conditional check on every allocation, reallocation and deallocation.
/// [`std::alloc::System`].
#[derive(Default)]
pub struct NvAllocator {
    alloc: System,
    preserve_exit: bool,
}

impl NvAllocator {
    pub const fn new(preserve_exit: bool) -> Self {
        Self { alloc: System, preserve_exit }
    }
}

unsafe impl GlobalAlloc for NvAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        unsafe {
            #[allow(unused_mut)]
            let mut ptr = self.alloc.alloc(layout);

            #[cfg(not(any(miri, test)))]
            if ptr.is_null() && can_call() {
                try_to_free_memory();
                ptr = self.alloc.alloc(layout);
            }
            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        unsafe { self.alloc.dealloc(ptr, layout) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        unsafe {
            #[allow(unused_mut)]
            let mut ptr = self.alloc.realloc(ptr, layout, new_size);
            #[cfg(not(any(miri, test)))]
            if ptr.is_null() && can_call() {
                try_to_free_memory();
                ptr = self.alloc.realloc(ptr, layout, new_size);
            }
            ptr
        }
    }

    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        unsafe {
            #[allow(unused_mut)]
            let mut ptr = self.alloc.alloc_zeroed(layout);
            #[cfg(not(any(miri, test)))]
            if ptr.is_null() && can_call() {
                try_to_free_memory();
                ptr = self.alloc.alloc_zeroed(layout);
            }

            ptr
        }
    }
}
