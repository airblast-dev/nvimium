use std::mem::MaybeUninit;

use crate::masked_builder;

use crate::nvim_types::lua::Function;
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
        // Manually implemented
        #[builder(skip)]
        callback: LuaRef,
        desc: ThinString<'a>,
        replace_keycodes: Boolean,
    }
}

impl<'a> SetKeymapOpts<'a> {
    pub fn callback<F: 'static + Fn(()) + Unpin>(&mut self, f: F) -> &mut Self {
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
