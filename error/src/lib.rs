#[cold]
#[inline(never)]
#[track_caller]
pub fn alloc_failed() -> ! {
    panic!("unable to alloc memory")
}

#[cold]
#[inline(never)]
#[track_caller]
pub const fn slice_error() -> ! {
    panic!("slice len > isize::MAX")
}
