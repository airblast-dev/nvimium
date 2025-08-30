use libc::c_int;
use mlua_sys::lua_pushinteger;

use crate::nvim_types::{HandleT, Integer, buffer::Buffer, lua::LuaInteger};

use super::{FromLua, FromLuaErr, IntoLua};

impl FromLua for Buffer {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        index: c_int,
        to_pop: &mut i32,
    ) -> super::Result<Self> {
        let int = unsafe { Integer::get(l, index, to_pop) }?;

        Ok(Self(
            // buffer arg should be at most [`i32::MAX`] so exceeding that value would mean
            // - something went deeply wrong in this library
            // - someone opened more than [`i32::MAX`] buffers (madman!)
            // - can also error on niche platforms where [`c_int`] is small but neovim doesn't officially support such platforms
            //   so problems are expected anyways
            HandleT::try_from(int).map_err(|_| FromLuaErr::IncorrectType)?,
        ))
    }
}

impl IntoLua for Buffer {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe { lua_pushinteger(l, self.0 as LuaInteger) };
    }
}
