use macros::masked_builder;

use crate::{Boolean, lua_ref::LuaRef};

// TODO: replace with manual builder
masked_builder! {
    #[repr(C)]
    pub struct OpenTermOpts {
        on_input: LuaRef,
        force_crlf: Boolean,
    }
}
