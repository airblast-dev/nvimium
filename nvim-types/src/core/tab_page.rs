use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct TabPage(HandleT);

impl TabPage {
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
