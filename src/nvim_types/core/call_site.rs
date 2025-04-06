#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Channel(i64);

impl Channel {
    // Simplified version of src/nvim/api/private/defs.h 0.10.0 l:42
    //
    // These constants are the values passed as "channel_id" to neovim.
    pub const INTERNAL_CALL_MASK: Channel = Channel(1 << 63);
    pub const VIML_INTERNAL_CALL: Channel = Channel(Self::INTERNAL_CALL_MASK.0);
    pub const LUA_INTERNAL_CALL: Channel = Channel(Self::VIML_INTERNAL_CALL.0 + 1);

    /// Construct a custom [`Channel`]
    ///
    /// You are generally discouraged from using this and the associated constants should be
    /// preferred instead.
    pub const fn new(chan: i64) -> Self {
        Self(chan)
    }

    /// Get the raw integer value of the [`Channel`]
    pub const fn as_int(self) -> i64 {
        self.0
    }
}
