#[repr(C)]
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
}

// TODO: use arena to optimize performance
// This is somewhat low priority but will be useful for large allocations
extern "C" {
    fn alloc_block() -> *mut libc::c_void;
    fn free_block() -> *mut libc::c_void;
}
