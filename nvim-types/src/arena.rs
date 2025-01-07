use std::{marker::PhantomData, ptr::addr_of_mut};

static mut ARENA: *mut Arena = core::ptr::null_mut();
// arena_alloc_block for allocating
#[repr(C)]
#[derive(Clone, Copy)]
struct Arena {
    cur_blk: *const libc::c_char,
    pos: libc::size_t,
    size: libc::size_t,
}

impl Arena {
    const EMPTY: Arena = Arena {
        cur_blk: std::ptr::null_mut(),
        pos: 0,
        size: 0,
    };

    fn new() -> Self {
        Self::EMPTY
    }

    unsafe fn alloc(&mut self, size: libc::size_t, align: bool) -> *mut libc::c_char {
        unsafe { arena_alloc(self as *mut Arena, size, align) }
    }

    unsafe fn finish(&mut self) -> ArenaMem {
        unsafe { arena_finish(self as *mut Arena) }
    }
}

#[repr(C)]
struct ArenaMem<'a>(*mut Self, PhantomData<&'a u8>);

impl ArenaMem<'_> {
    unsafe fn clone(&self) -> Self {
        ArenaMem(self.0, PhantomData::<&'static u8>)
    }
}

impl Drop for ArenaMem<'_> {
    fn drop(&mut self) {
        unsafe { arena_mem_free(self.clone()) };
    }
}

// TODO: use arena to optimize performance
// This is somewhat low priority but will be useful for large allocations
extern "C" {
    fn arena_alloc(arena: *mut Arena, size: libc::size_t, align: bool) -> *mut libc::c_char;
    fn arena_finish<'a>(arena: *mut Arena) -> ArenaMem<'a>;
    fn arena_mem_free(arena_mem: ArenaMem);
}
