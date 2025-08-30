use super::HandleT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NameSpace(HandleT);

impl NameSpace {
    /// Initialize a new [`NameSpace`]
    pub const fn new(id: HandleT) -> Self {
        Self(id)
    }

    /// Get the raw integer value of the [`NameSpace`]
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
