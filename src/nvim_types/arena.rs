use std::ptr::{NonNull, null_mut};

use libc::{c_char, size_t};
use thread_lock::call_check;

// arena_alloc_block for allocating
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Arena {
    cur_blk: Option<NonNull<c_char>>,
    pos: size_t,
    size: size_t,
}

const _: () = assert!(size_of::<Arena>() == size_of::<*mut c_char>() + size_of::<size_t>() * 2);

impl Arena {
    pub const EMPTY: Arena = Arena {
        cur_blk: None,
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
#[derive(Clone, Debug)]
pub struct ArenaMem(*mut Self);

impl Drop for ArenaMem {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let mem = self.0;
            self.0 = null_mut();
            unsafe { arena_mem_free(Self(mem)) };
        }
    }
}

unsafe extern "C" {
    pub(crate) fn arena_mem_free(arena_mem: ArenaMem);
    pub(crate) fn arena_finish(arena: *mut Arena) -> ArenaMem;
}
