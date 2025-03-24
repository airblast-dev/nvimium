use mlua_sys::{LUA_TNONE, LUA_TNUMBER, lua_tointeger, lua_tonumber, lua_type};

use crate::{Float, Integer};

use super::{FromLua, FromLuaErr};

impl FromLua for Integer {
    unsafe fn pop(l: *mut mlua_sys::lua_State, idx: std::ffi::c_int) -> super::Result<Self> {
        let ty = unsafe { lua_type(l, idx) };
        if ty == LUA_TNONE {
            return Err(FromLuaErr::NotFound);
        }
        if ty != LUA_TNUMBER {
            return Err(FromLuaErr::IncorrectType);
        }

        // on 32 bit platforms tointeger returns an i32 so just cast it
        Ok(unsafe { lua_tointeger(l, idx) as Self })
    }
}

impl FromLua for Float {
    unsafe fn pop(l: *mut mlua_sys::lua_State, idx: std::ffi::c_int) -> super::Result<Self> {
        let ty = unsafe { lua_type(l, idx) };
        if ty == LUA_TNONE {
            return Err(FromLuaErr::NotFound);
        }
        if ty != LUA_TNUMBER {
            return Err(FromLuaErr::IncorrectType);
        }

        Ok(unsafe { lua_tonumber(l, idx) })
    }
}
