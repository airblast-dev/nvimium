use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window(HandleT);

impl Window {
    /// Initialize a new [`Window`]
    ///
    /// Prefer using window's returned by neovim instead of this function.
    pub const fn new(id: HandleT) -> Self {
        Self(id)
    }

    /// Get the raw integer value of the [`Window`]
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
