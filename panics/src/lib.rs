#[cold]
#[inline(never)]
#[track_caller]
pub fn alloc_failed() -> ! {
    // TODO: maybe use [`std::alloc::handle_alloc_error`]
    panic!("unable to allocate memory")
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
