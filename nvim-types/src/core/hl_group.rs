use super::HLGroupIDT;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct HlGroupId(HLGroupIDT);
