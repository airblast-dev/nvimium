use std::{cell::UnsafeCell, ffi::c_void, ptr::NonNull};

use libc::{c_char, c_double, size_t};
use thread_lock::call_check;

use crate::nvim_types::Boolean;

/// A block of memory provided by neovim
///
/// This does not take ownership of the block of memory but is only provided mutable access to said
/// block.
// neovim uses an arena for a sane alloc dealloc strategy
//
// the value at cur_blk is always a pointer to the previous block of memory or null
// a previous block is(should) only (be) present where an allocation exceeds the 4096 byte default capacity of an arena
// if the allocation did fit in 4096 bytes, the first value is a null pointer
// TODO: allow for recursive calls and nestedness tracking
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Arena {
    // TODO: make this public after adding cur block + c_char ptr
    cur_blk: Option<NonNull<c_char>>,
    pub pos: size_t,
    pub size: size_t,
}

const _: () = assert!(size_of::<Arena>() == size_of::<*mut c_char>() + size_of::<size_t>() * 2);

impl Arena {
    const MAX_HEAD_SIZE: size_t = arena_align_offset(size_of::<ArenaMem>() as u64);
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
    // this technically a hack as we are supposed to pass it to arena_mem_free and recreate an
    // arena but this has less friction as we aren't touching any neovim statics.
    // might also optimize things a bit as the compiler might be able to know what state our arena
    // is in
    pub(crate) fn reset_pos(&mut self) {
        self.pos = Self::MAX_HEAD_SIZE;
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
const fn arena_align_offset(off: u64) -> size_t {
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

pub(crate) struct TrackedArena {
    pub(crate) is_nested: bool,
    arena: Arena,
}
impl TrackedArena {
    /// # Safety
    ///
    /// Mutates a static mut by taking a mutable reference to it
    /// This call should be used to discard the arena on the top level call. This means that
    /// there cannot be other mutable references present at the point this is called.
    pub(crate) unsafe fn reset_arena(&mut self) {
        self.arena = Arena::EMPTY;
    }

    pub(crate) unsafe fn reset_pos(ta: *mut Self) {
        if !unsafe { (&raw const (*ta).is_nested).read() } {
            // SAFETY: no other mutable reference exists and ta is always a pointer to a static
            unsafe {
                ta.as_mut().unwrap_unchecked().arena.reset_pos();
            }
        };
    }
}

pub(crate) unsafe fn call_with_arena<R, F: FnOnce(*mut Arena) -> R>(f: F) -> R {
    unsafe {
        let ret;
        {

            let _drop_lock = NestRestore((&raw mut TRACKED_ARENA.is_nested).read());
            // in case of panics we ensure that
            struct NestRestore(Boolean);
            impl Drop for NestRestore {
                fn drop(&mut self) {
                    unsafe { (&raw mut TRACKED_ARENA.is_nested).write(self.0) };
                }
            }
            (&raw mut TRACKED_ARENA.is_nested).write(true);
            let arena = &raw mut TRACKED_ARENA.arena;
            ret = f(arena);
        }
        TrackedArena::reset_pos(&raw mut TRACKED_ARENA);
        ret
    }
}

pub(crate) static mut TRACKED_ARENA: TrackedArena = TrackedArena {
    is_nested: false,
    arena: Arena::EMPTY,
};

unsafe extern "C" {
    pub(crate) fn arena_mem_free(arena_mem: ArenaMem);
    pub(crate) fn arena_finish(arena: *mut Arena) -> ArenaMem;
}
