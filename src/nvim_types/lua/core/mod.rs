//! Module for converting lua types to neovim types and vice versa
mod array;
mod boolean;
mod buffer;
mod dict;
mod hl_group;
mod integer;
mod lua_ref;
mod namespace;
mod object;
mod string;
mod tabpage;
mod window;

use mlua_sys::{lua_State, lua_pushnil};

pub trait FromLua: 'static + Sized {
    unsafe fn pop(l: *mut lua_State) -> Result<Self>;
}

pub(crate) type Result<T> = core::result::Result<T, FromLuaErr>;

#[derive(Clone, Copy, Debug)]
pub enum FromLuaErr {
    NotFound,
    IncorrectType,
}

impl<T: FromLua> FromLua for Option<T> {
    unsafe fn pop(l: *mut lua_State) -> Result<Self> {
        match unsafe { T::pop(l) } {
            Ok(t) => Ok(Some(t)),
            Err(FromLuaErr::NotFound) => Ok(None),
            _ => Err(FromLuaErr::IncorrectType),
        }
    }
}

pub trait IntoLua {
    /// Pushes a lua value onto the stack
    ///
    /// # Safety
    ///
    /// Every call must push a value even if it is a nill. Not doing so can result in UB in some
    /// cases.
    unsafe fn push(&self, l: *mut lua_State);
}

impl IntoLua for () {
    unsafe fn push(&self, l: *mut lua_State) {
        unsafe { lua_pushnil(l) };
    }
}
