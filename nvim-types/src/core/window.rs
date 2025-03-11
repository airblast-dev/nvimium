use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Window(HandleT);

impl Window {
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
