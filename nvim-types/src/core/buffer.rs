use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Buffer(HandleT);

impl Buffer {
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
