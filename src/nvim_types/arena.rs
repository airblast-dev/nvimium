use std::{
    cell::{RefCell, UnsafeCell},
    ffi::c_void,
    mem::MaybeUninit,
    ptr::NonNull,
};

use libc::{c_char, c_double, size_t};
use thread_lock::call_check;

/// A block of memory provided by neovim
///
/// This does not take ownership of the block of memory but is only provided mutable access to said
/// block.
// neovim uses an arena for a sane alloc dealloc strategy
//
// the value at cur_blk is always a pointer to the previous block of memory or null
// a previous block is(should) only (be) present where an allocation exceeds the 4096 byte default capacity of an arena
// if the allocation did fit in 4096 bytes, the first value is a null pointer
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Arena {
    // TODO: make this public after adding cur block + c_char ptr
    cur_blk: Option<NonNull<c_char>>,
    pub pos: size_t,
    pub size: size_t,
}

thread_local! {
    /// An [`Arena`] struct that is reused between neovim calls
    ///
    /// Acquiring mutable access to the arena and calling a neovim function in the scope of mutable
    /// access will cause a panic. Instead mutable access to the callback arena and neovim calls should
    /// be done in different scopes to avoid a panic.
    ///
    /// Even if some functions may not use this arena, the library assumes that all neovim functions can acquire mutable access
    /// and any change making a neovim function to make use of the arena is not considered a breaking.
    pub static CALLBACK_ARENA: RefCell<Arena> = const { RefCell::new(Arena::EMPTY) };
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

    /// Get a view of the initialized portion of the arena as a slice of bytes
    ///
    /// Returns [`None`] if no block has been acquired or the position of our block is less than a
    /// [`size_t`].
    pub fn as_bytes(&self) -> Option<&[u8]> {
        self.cur_blk.and_then(|ptr| unsafe {
            if self.pos >= Self::MAX_HEAD_SIZE {
                let start = ptr.add(Self::MAX_HEAD_SIZE);
                Some(std::slice::from_raw_parts(
                    start.as_ptr() as *const u8,
                    self.pos - Self::MAX_HEAD_SIZE,
                ))
            } else {
                None
            }
        })
    }

    pub fn spare_capacity(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe {
            self.cur_blk
                .map(|blk| {
                    NonNull::slice_from_raw_parts(
                        blk.add(self.pos).cast::<MaybeUninit<u8>>(),
                        self.size.saturating_sub(self.pos),
                    )
                    .as_mut()
                })
                .unwrap_or(&mut [])
        }
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

unsafe extern "C" {
    pub(crate) fn arena_mem_free(arena_mem: ArenaMem);
    pub(crate) fn arena_finish(arena: *mut Arena) -> ArenaMem;
}
