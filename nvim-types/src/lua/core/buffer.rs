
use crate::{buffer::Buffer, HandleT, Integer};

use super::{FromLua, FromLuaErr};

impl FromLua for Buffer {
    unsafe fn pop(l: *mut mlua_sys::lua_State, idx: std::ffi::c_int) -> super::Result<Self> {
        let int = unsafe { Integer::pop(l, idx) }?;

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
