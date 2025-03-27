//! Module for converting lua types to neovim types and vice versa
mod boolean;
mod buffer;
mod integer;
mod string;

use std::ffi::c_int;

use mlua_sys::lua_State;

pub trait FromLuaMulti: 'static + Sized {
    unsafe fn pop(l: *mut lua_State) -> Result<Self>;
}

impl<T: FromLua> FromLuaMulti for T {
    unsafe fn pop(l: *mut lua_State) -> Result<Self> {
        unsafe { <Self as FromLua>::pop(l, -1) }
    }
}

pub(crate) trait FromLua: 'static + Sized {
    unsafe fn pop(l: *mut lua_State, idx: c_int) -> Result<Self>;
}

pub(crate) type Result<T> = core::result::Result<T, FromLuaErr>;

#[derive(Clone, Copy, Debug)]
pub(crate) enum FromLuaErr {
    NotFound,
    IncorrectType,
}

impl<T: FromLua> FromLua for Option<T> {
    unsafe fn pop(l: *mut lua_State, idx: c_int) -> Result<Self> {
        match unsafe { T::pop(l, idx) } {
            Ok(t) => Ok(Some(t)),
            Err(FromLuaErr::NotFound) => Ok(None),
            _ => Err(FromLuaErr::IncorrectType),
        }
    }
}

pub(crate) trait IntoLua: 'static {
    unsafe fn push(l: *mut lua_State);
}

impl<T: IntoLua> IntoLuaMulti for T {
    unsafe fn push(l: *mut lua_State) {
        unsafe { <Self as IntoLua>::push(l) };
    }
}

pub trait IntoLuaMulti: 'static + Sized {
    unsafe fn push(l: *mut lua_State);
}
