//! Module for converting lua types to neovim types and vice versa
mod boolean;
mod buffer;
mod dict;
mod integer;
mod string;

use mlua_sys::lua_State;

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
    unsafe fn push(&self, l: *mut lua_State);
}

impl IntoLua for () {
    unsafe fn push(&self, _: *mut lua_State) {}
}
