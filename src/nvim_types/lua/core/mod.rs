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

use std::{error::Error, fmt::Display};

use libc::c_int;
use mlua_sys::{lua_State, lua_pop};

pub trait FromLuaMany: Sized {
    unsafe fn get(l: *mut lua_State, to_pop: &mut i32) -> Result<Self>;
}
impl<T> FromLuaMany for T
where
    T: FromLua,
{
    unsafe fn get(l: *mut lua_State, to_pop: &mut i32) -> Result<Self> {
        unsafe { <Self as FromLua>::get(l, -1, to_pop) }
    }
}
pub trait FromLua: 'static + Sized {
    unsafe fn pop(l: *mut lua_State, index: c_int) -> Result<Self> {
        unsafe {
            let mut to_pop = 0;
            let ret = <Self as FromLua>::get(l, index, &mut to_pop)?;
            lua_pop(l, to_pop);
            Ok(ret)
        }
    }
    unsafe fn get(l: *mut lua_State, index: c_int, to_pop: &mut i32) -> Result<Self>;
}

pub(crate) type Result<T> = core::result::Result<T, FromLuaErr>;

#[derive(Clone, Copy, Debug)]
pub enum FromLuaErr {
    NotFound,
    IncorrectType,
    NotEnoughStackSpace,
}

impl Display for FromLuaErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotFound => "field not found",
            Self::IncorrectType => "incorrect lua type found",
            Self::NotEnoughStackSpace => "not enough stack space to read lua values",
        };
        write!(f, "{}", s)
    }
}

impl Error for FromLuaErr {}

impl<T: FromLua> FromLua for Option<T> {
    unsafe fn get(l: *mut lua_State, index: c_int, to_pop: &mut i32) -> Result<Self> {
        match unsafe { T::get(l, index, to_pop) } {
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
    unsafe fn push(&self, _: *mut lua_State) {}
}

impl FromLua for () {
    unsafe fn get(_l: *mut lua_State, _index: c_int, _to_pop: &mut i32) -> Result<Self> {
        Ok(())
    }
}
