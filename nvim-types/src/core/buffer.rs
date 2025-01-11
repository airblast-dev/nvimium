use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Buffer(HandleT);
