use std::ffi::c_void;

use thread_lock::call_check;

// arena_alloc_block for allocating
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Arena {
    cur_blk: *mut libc::c_char,
    pos: libc::size_t,
    size: libc::size_t,
}

impl Arena {
    pub const EMPTY: Arena = Arena {
        cur_blk: core::ptr::null_mut(),
        pos: 0,
        size: 0,
    };
}

impl Drop for Arena {
    fn drop(&mut self) {
        // SAFETY: we are mutating a neovim static in arena_finish, we must have execution yielded
        // in order to drop this
        //
        // it might make more sense to just leak instead of a panic but the allocated space of 
        // arena's are somewhat large
        call_check();
        unsafe { arena_finish(self) };
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct ArenaMem(*mut Self);

impl Drop for ArenaMem {
    fn drop(&mut self) {
        unsafe { arena_mem_free(self.clone()) };
    }
}

// TODO: use arena to optimize performance
// This is somewhat low priority but will be useful for large allocations
// NOTE: actually seems some api's kind of require this in order to provide a sane deallocation
unsafe extern "C" {
    pub(crate) fn arena_mem_free(arena_mem: ArenaMem);
    pub(crate) fn arena_finish(arena: *mut Arena) -> ArenaMem;
    pub(crate) fn arena_alloc(arena: *mut Arena, size: usize, align: bool) -> *mut c_void;
}
