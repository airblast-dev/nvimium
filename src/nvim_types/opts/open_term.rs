use std::mem::MaybeUninit;

use thread_lock::get_lua_ptr;

use crate::masked_builder;

use crate::nvim_types::args::open_term_cb::OpenTermOnInputArgs;
use crate::nvim_types::lua::Function;
use crate::nvim_types::{Boolean, lua_ref::LuaRef};

// TODO: replace with manual builder
masked_builder! {
    #[repr(C)]
    pub struct OpenTermOpts {
        #[builder(skip)]
        on_input: LuaRef,
        force_crlf: Boolean,
    }
}

impl OpenTermOpts {
    pub fn on_input<F: 'static + for<'a> Fn(OpenTermOnInputArgs<'a>) + Unpin>(
        &mut self,
        f: F,
    ) -> &mut Self {
        // OpenTermOnInputArgs contains references so we cannot use FromLua
        // instead we provide 'a to limit the scope that the fields are valid.
        let cb = Function::wrap(move |_: ()| {
            f(unsafe { OpenTermOnInputArgs::pop(get_lua_ptr().as_ptr()).unwrap() })
        });
        if self.mask & 2 == 2 {
            unsafe { self.on_input.assume_init_drop() };
        }
        self.mask |= 2;
        self.on_input = MaybeUninit::new(cb.into_luaref());
        self
    }
}
