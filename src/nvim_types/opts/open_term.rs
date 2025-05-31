use std::error::Error;
use std::mem::MaybeUninit;

use crate::macros::masked_builder::masked_builder;
use crate::nvim_types::args::open_term_cb::OpenTermOnInputArgs;
use crate::nvim_types::lua::Function;
use crate::nvim_types::{Boolean, lua_ref::LuaRef};

// TODO: replace with manual builder
masked_builder! {
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
        f: impl 'static + for<'a> Fn(OpenTermOnInputArgs<'a>) -> Result<(), E> + Unpin,
    ) -> &mut Self {
        let cb = Function::wrap(f);
        if self.mask & 2 == 2 {
            unsafe { self.on_input.assume_init_drop() };
        }
        self.mask |= 2;
        self.on_input = MaybeUninit::new(cb.into_luaref());
        self
    }
}
