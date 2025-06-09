use std::error::Error;
use std::mem::MaybeUninit;

use crate::macros::decl_derive::derive;
use crate::nvim_types::args::open_term_cb::OpenTermOnInputArgs;
use crate::nvim_types::lua::{Function, NvFn};
use crate::nvim_types::{Boolean, lua_ref::LuaRef};

derive! {
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    pub struct OpenTermOpts {
        #[builder_fn_skip]
        on_input: LuaRef,
        force_crlf: Boolean,
    }
}

impl OpenTermOpts {
    pub fn on_input<E: 'static + Error>(
        &mut self,
        f: impl 'static + NvFn + for<'a> Fn(OpenTermOnInputArgs<'a>) -> Result<(), E>,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[0];
        let cb = Function::wrap(f);
        if self.mask & MASK == MASK {
            unsafe { self.on_input.assume_init_drop() };
        }
        self.mask |= MASK;
        self.on_input = MaybeUninit::new(cb.into_luaref());
        self
    }
}
