use libc::c_int;
use mlua_sys::{
    LUA_TNONE, LUA_TNUMBER, lua_checkstack, lua_pushinteger, lua_pushnumber, lua_tointeger,
    lua_tonumber, lua_type,
};

use crate::nvim_types::{Float, Integer};

use super::{FromLua, FromLuaErr, IntoLua};

impl FromLua for Integer {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        index: c_int,
        to_pop: &mut i32,
    ) -> super::Result<Self> {
        let ty = unsafe { lua_type(l, index) };
        if ty == LUA_TNONE {
            return Err(FromLuaErr::NotFound);
        }
        *to_pop += 1;
        if ty != LUA_TNUMBER {
            return Err(FromLuaErr::IncorrectType);
        }

        // on 32 bit platforms tointeger returns an i32 so just cast it
        Ok(unsafe { lua_tointeger(l, index) as Self })
    }
}

impl IntoLua for Integer {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe {
            lua_checkstack(l, 1);
            lua_pushinteger(l, *self);
        }
    }
}

impl FromLua for Float {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        index: c_int,
        to_pop: &mut i32,
    ) -> super::Result<Self> {
        let ty = unsafe { lua_type(l, index) };
        if ty == LUA_TNONE {
            return Err(FromLuaErr::NotFound);
        }
        *to_pop += 1;
        if ty != LUA_TNUMBER {
            return Err(FromLuaErr::IncorrectType);
        }

        Ok(unsafe { lua_tonumber(l, index) })
    }
}

impl IntoLua for Float {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe {
            lua_checkstack(l, 1);
            lua_pushnumber(l, *self);
        };
    }
}
