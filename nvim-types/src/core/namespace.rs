use super::HandleT;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct NameSpace(HandleT);

impl NameSpace {
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
