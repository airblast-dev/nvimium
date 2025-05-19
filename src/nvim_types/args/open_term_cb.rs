use mlua_sys::{lua_pop, lua_tolstring, lua_type, lua_typename};

use crate::nvim_types::{Buffer, FromLua, Integer, String, ThinString, lua::core::FromLuaMany};

pub struct OpenTermOnInputArgs<'a> {
    pub src: ThinString<'a>,
    pub term: Integer,
    pub buf: Buffer,
    pub data: ThinString<'a>,
}

impl<'a> FromLuaMany for OpenTermOnInputArgs<'a> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            Ok(Self {
                src: ThinString::get(l, -4, to_pop)?,
                term: <Integer as FromLua>::get(l, -3, to_pop)?,
                buf: <Buffer as FromLua>::get(l, -2, to_pop)?,
                data: ThinString::get(l, -1, to_pop)?,
            })
        }
    }
}
