use macros::masked_builder;

use crate::{lua_ref::LuaRef, string::ThinString, Boolean};

masked_builder! {
    #[repr(C)]
    pub struct SetKeymapOpts<'a> {
        noremap: Boolean,
        noawait: Boolean,
        silent: Boolean,
        script: Boolean,
        expr: Boolean,
        unique: Boolean,
        // skip for now until lua is supported
        #[builder(skip)]
        callback: LuaRef,
        desc: ThinString<'a>,
        replace_keycodes: Boolean,
    }
}
