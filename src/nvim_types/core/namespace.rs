use super::HandleT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NameSpace(HandleT);

impl NameSpace {
    pub const fn new(id: HandleT) -> Self {
        Self(id)
    }

    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
