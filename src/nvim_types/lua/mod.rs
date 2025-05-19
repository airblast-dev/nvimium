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
mod closure;
pub mod core;
mod fn_ptr;
#[doc(hidden)]
pub use box_fn::set_callback_name;

use core::FromLuaMany;
pub use core::{FromLua, IntoLua};
use std::any::Any;

#[doc(hidden)]
pub use mlua_sys::lua_State;
use thread_lock::get_lua_ptr;

use crate::nvim_types::LuaRef;

#[cfg(target_pointer_width = "32")]
type LuaInteger = i32;
#[cfg(target_pointer_width = "64")]
type LuaInteger = i64;

#[repr(transparent)]
pub struct Function(LuaRef);

impl Function {
    pub(crate) fn from_fn<F: 'static + Fn(A) -> R + Unpin, A: FromLua, R: IntoLua>(f: F) -> Self {
        let mut l = get_lua_ptr();
        Self(unsafe { LuaRef::new(closure::register(l.as_ptr(), f)) })
    }

    pub(crate) fn from_fn_ptr<A: FromLuaMany, R: IntoLua>(f: fn(A) -> R) -> Self {
        let mut l = get_lua_ptr();
        Self(unsafe { LuaRef::new(fn_ptr::register(l.as_ptr(), f)) })
    }

    pub(crate) fn from_box_fn<F: 'static + Fn(A) -> R, A: FromLuaMany, R: IntoLua>(f: F) -> Self {
        let mut l = get_lua_ptr();
        Self(unsafe { LuaRef::new(box_fn::register(l.as_ptr(), f)) })
    }

    pub fn into_luaref(self) -> LuaRef {
        self.0
    }

    /// Wraps the provided function and returns a [`Function`] that stores the key to the lua registry
    ///
    /// The provided function will be attempted to be downcasted to a function pointer for cheaper
    /// initialization and drops.
    pub fn wrap<F: 'static + Fn(A) -> R + Unpin, A: 'static + FromLuaMany, R: 'static + IntoLua>(
        f: F,
    ) -> Self {
        // if F is a function pointer we can avoid dynamic dispatch, an extra indirection and the drop
        // call passed to lua (we can use lightuserdata)
        if let Some(f) = (&f as &dyn Any).downcast_ref::<fn(A) -> R>() {
            Self::from_fn_ptr(*f)
        } else {
            Self::from_box_fn(f)
        }
    }
}
