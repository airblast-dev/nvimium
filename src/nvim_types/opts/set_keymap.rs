use std::error::Error;
use std::mem::MaybeUninit;

use crate::macros::decl_derive::derive;
use crate::nvim_types::lua::{Function, NvFn};
use crate::nvim_types::{Boolean, lua_ref::LuaRef, string::ThinString};

derive! {
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    pub struct SetKeymapOpts<'a> {
        noremap: Boolean,
        noawait: Boolean,
        silent: Boolean,
        script: Boolean,
        expr: Boolean,
        unique: Boolean,
        // Manually implemented
        #[builder_fn_skip]
        callback: LuaRef,
        desc: ThinString<'a>,
        replace_keycodes: Boolean,
    }
}

impl<'a> SetKeymapOpts<'a> {
    pub fn callback<E: 'static + Error, F: 'static + NvFn + Fn(()) -> Result<(), E>>(
        &mut self,
        f: F,
    ) -> &mut Self {
        let lref = Function::wrap(f).into_luaref();
        const CB_MASK: u64 = 1 << 8;
        if self.mask & CB_MASK == CB_MASK {
            unsafe { self.callback.assume_init_drop() };
        }
        self.mask |= CB_MASK;
        self.callback = MaybeUninit::new(lref);
        self
    }
}
