#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Channel(u64);

impl Channel {
    // Simplified version of src/nvim/api/private/defs.h 0.10.0 l:42
    //
    // These constants are the values passed as "channel_id" to neovim.
    pub const INTERNAL_CALL_MASK: Channel = Channel(1 << 63);
    pub const VIML_INTERNAL_CALL: Channel = Channel(Self::INTERNAL_CALL_MASK.0);
    pub const LUA_INTERNAL_CALL: Channel = Channel(Self::VIML_INTERNAL_CALL.0 + 1);

    pub const fn as_int(self) -> u64 {
        self.0
    }
}
