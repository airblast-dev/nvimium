// Simplified version of src/nvim/api/private/defs.h 0.10.0 l:42
//
// These constants are the values passed as "channel_id" to neovim.
pub const INTERNAL_CALL_MASK: u64 = (1) << (63);
pub const VIML_INTERNAL_CALL: u64 = INTERNAL_CALL_MASK;
pub const LUA_INTERNAL_CALL: u64 = VIML_INTERNAL_CALL + 1;
