use std::{
    cell::{RefCell, UnsafeCell},
    ffi::c_void,
    ptr::NonNull,
};

use libc::{c_char, c_double, size_t};
use thread_lock::call_check;

// arena_alloc_block for allocating
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Arena {
    // TODO: make this public after adding cur block + c_char ptr
    cur_blk: Option<NonNull<c_char>>,
    pub pos: size_t,
    pub size: size_t,
}

thread_local! {
    pub static CALLBACK_ARENA: RefCell<Arena> = const { RefCell::new(Arena::EMPTY) };
}
const _: () = assert!(size_of::<Arena>() == size_of::<*mut c_char>() + size_of::<size_t>() * 2);

impl Arena {
    pub const EMPTY: Arena = Arena {
        cur_blk: None,
        pos: 0,
        size: 0,
    };

    /// Reset the position of the arena
    ///
    /// Unlike setting the `pos` field to 0 this will minimally keep enough space to store the
    /// previous block information. In case you are using an arena and want to reclaim space
    /// without dealing with neovim functions use this.
    ///
    /// While this function is safe, incorrect usage of the Arena after calling this function can
    /// cause UB.
    pub fn reset_pos(&mut self) {
        self.pos = arena_align_offset(size_of::<ArenaMem>() as u64);
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        // SAFETY: we are mutating a neovim static in arena_finish, we must have execution yielded
        // in order to drop this
        if self.cur_blk.is_some() {
            call_check();
            unsafe { arena_mem_free(arena_finish(self)) };
        }
    }
}

pub type ArenaMem = UnsafeCell<*mut ConsumedBlk>;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ConsumedBlk {
    prev: *mut Self,
}

/// The minimum space needed to store the prev block ptr.
fn arena_align_offset(off: u64) -> size_t {
    // no idea why we do this but just we are just replicating whats done in the same function in
    // neovim
    const ARENA_ALIGN: usize = {
        if size_of::<c_double>() >= size_of::<*mut c_void>() {
            size_of::<c_double>()
        } else {
            size_of::<*mut c_void>()
        }
    };

    // instead of the size of ArenaMem we might be able to find a way to recover these precious
    // bytes where a pointer isn't stored in it
    //
    // sadly difficult to do as we have no way to know how the allocation was requested in neovim
    // (namely the align value passed to allocator functions)
    //
    // worst case we unnecesarily use the size of a pointer
    (off as usize + (ARENA_ALIGN - 1)) & !(ARENA_ALIGN - 1)
}

unsafe extern "C" {
    pub(crate) fn arena_mem_free(arena_mem: ArenaMem);
    pub(crate) fn arena_finish(arena: *mut Arena) -> ArenaMem;
}
