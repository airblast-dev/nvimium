use super::HLGroupIDT;

/// A highlight group handle
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct HlGroupId(HLGroupIDT);

impl HlGroupId {
    /// Initialize a new [`HlGroupId`]
    ///
    /// Prefer using highlight groups returned by neovim or the associated constants instead of
    /// this function.
    pub const fn new(h: HLGroupIDT) -> Self {
        Self(h)
    }

    /// The raw integer value of the [`HlGroupId`]
    pub fn as_int(&self) -> HLGroupIDT {
        self.0
    }
}
