#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Window(libc::c_int);
