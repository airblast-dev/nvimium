use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabPage(HandleT);

impl TabPage {
    pub const fn new(id: HandleT) -> Self {
        Self(id)
    }

    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
