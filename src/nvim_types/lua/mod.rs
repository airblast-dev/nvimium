// Using mlua instead of mlua-sys would remove the need of many of the things in this module
//
// Problem is the function struct it provides works via the lua stack and not with the lua
// registry. This means we cannot pass a function to neovim via mlua.
//
// Instead this is a partial reimplimentation of the functionality of mlua but aims to only support
// callbacks passed to neovim.
//
// The goal here is to provide the minimum requirements to be able to effectively use the C api.
// If lua functions are intended to be called directly users should use mlua in combination with
// nvimium. In some cases nvimium may provide convenience traits to simplify things but this is not
// a guarantee as some things in neovim dont map well to mlua's API (mlua is really great, just not
// suitable here).
//
// while some parts could still be used from mlua it is not really worth bringing in another dependency
// as all use cases have a fix set of arguments which we can handle internally
mod box_fn;
pub mod core;
pub(crate) mod utils;

// used from a plugin! macro in user crate
#[doc(hidden)]
pub use box_fn::set_callback_name;

use core::FromLuaMany;
pub use core::{FromLua, IntoLua};
use std::{
    error::Error,
    panic::{RefUnwindSafe, UnwindSafe},
};

#[doc(hidden)]
pub use mlua_sys::lua_State;
use thread_lock::get_lua_ptr;

use crate::nvim_types::LuaRef;

#[cfg(target_pointer_width = "32")]
pub(crate) type LuaInteger = i32;
#[cfg(target_pointer_width = "64")]
pub(crate) type LuaInteger = i64;

#[repr(transparent)]
pub struct Function(LuaRef);

impl Function {
    pub(crate) fn from_box_fn<
        E: Error,
        F: 'static + Fn(A) -> Result<R, E>,
        A: FromLuaMany,
        R: IntoLua,
    >(
        f: F,
    ) -> Self {
        let mut l = get_lua_ptr();
        Self(unsafe { LuaRef::new(box_fn::register(l.as_ptr(), f)) })
    }

    pub fn into_luaref(self) -> LuaRef {
        self.0
    }

    /// Wraps the provided function and passes it to Lua
    ///
    /// [`Function::wrap`] will pass the function to Lua and return a [`Function`] containing the
    /// Lua reference.
    pub fn wrap<
        A: 'static + FromLuaMany,
        R: 'static + IntoLua,
        E: 'static + Error,
        F: 'static + Fn(A) -> Result<R, E> + Unpin,
    >(
        f: F,
    ) -> Self {
        Self::from_box_fn(f)
    }
}

pub trait NvFn: 'static + Unpin + UnwindSafe + RefUnwindSafe + Send + Sync {}
impl<T: 'static + Unpin + UnwindSafe + RefUnwindSafe + Send + Sync> NvFn for T {}
