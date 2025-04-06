use super::HLGroupIDT;

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct HlGroupId(HLGroupIDT);

impl HlGroupId {
    pub fn as_int(&self) -> HLGroupIDT {
        self.0
    }
}
