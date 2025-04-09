use crate::masked_builder;

use crate::nvim_types::{Boolean, lua_ref::LuaRef, string::ThinString};

masked_builder! {
    #[repr(C)]
    pub struct SetKeymapOpts<'a> {
        noremap: Boolean,
        noawait: Boolean,
        silent: Boolean,
        script: Boolean,
        expr: Boolean,
        unique: Boolean,
        // TODO: skip for now until lua is supported
        #[builder(skip)]
        callback: LuaRef,
        desc: ThinString<'a>,
        replace_keycodes: Boolean,
    }
}
