use super::HandleT;

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Buffer(pub(crate) HandleT);

impl Buffer {
    /// Initialize a new [`Buffer`]
    ///
    /// Prefer using values returned by neovim instead of initializing the type manually.
    ///
    /// Some functions that neovim provides use a zero ID buffer as the current buffer, some dont
    /// strictly document it. If the called function is known to accept the zero value this can be
    /// initialized using `0` for the current buffer.
    pub const fn new(h: HandleT) -> Self {
        Self(h)
    }

    /// The raw integer value of the [`Buffer`]
    pub const fn as_int(&self) -> HandleT {
        self.0
    }
}
