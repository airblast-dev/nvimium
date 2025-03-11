use macros::masked_builder;

use crate::{Boolean, LuaRef};

// TODO: replace with manual builder
masked_builder!{
    #[repr(C)]
    #[derive(Debug)]
    pub struct OpenTermOpts {
        on_input: LuaRef,
        force_crlf: Boolean,
    }
}
