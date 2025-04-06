use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TabPage(HandleT);

impl TabPage {
    /// Initialize a new [`TabPage`]
    ///
    /// Prefer using tabpage's returned by neovim instead of this function.
    pub const fn new(id: HandleT) -> Self {
        Self(id)
    }

    /// Get the raw integer value of the [`TabPage`]
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
