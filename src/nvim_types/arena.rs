use core::marker::PhantomData;
use std::ffi::c_void;

// arena_alloc_block for allocating
#[repr(C)]
#[derive(Clone, Copy, Debug)]
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

#[repr(C)]
#[derive(Clone)]
pub struct ArenaMem(*mut Self);

impl ArenaMem {
    unsafe fn drop(&mut self) {
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
