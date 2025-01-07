// TODO: add more recovery/abort functions
//
// see [`alloc_failed`] for example usage
extern "C" {
    /// Neovim's custom abort 
    ///
    /// Does some cleanup and logging before crashing. 
    /// Generally preferred over a segfault.
    ///
    /// # Safety
    ///
    /// err_msg should only point to null, or a string terminated with a null byte.
    fn preserve_exit(err_msg: *const libc::c_char) -> !;
}

#[cold]
#[inline(never)]
#[track_caller]
pub fn alloc_failed() -> ! {
    const E_OUTOFMEM: *const libc::c_char = c"E41: Out of memory!".as_ptr();

    unsafe { preserve_exit(E_OUTOFMEM) }
}

#[cold]
#[inline(never)]
#[track_caller]
pub const fn slice_error() -> ! {
    panic!("slice len should never be > isize::MAX")
}

#[cold]
#[track_caller]
#[inline(never)]
pub const fn not_null_terminated(l: Option<u8>) -> ! {
    if l.is_some() {
        panic!("provided bytes should always be null terminated");
    } else {
        panic!("provided bytes length should always be > 0")
    }
}
