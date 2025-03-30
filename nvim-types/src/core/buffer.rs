use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Buffer(pub(crate) HandleT);

impl Buffer {
    pub const fn new(h: HandleT) -> Self {
        Self(h)
    }
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
