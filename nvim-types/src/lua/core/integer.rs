use mlua_sys::{
    LUA_TNONE, LUA_TNUMBER, lua_checkstack, lua_pushinteger, lua_pushnumber, lua_tointeger,
    lua_tonumber, lua_type,
};

use crate::{Float, Integer};

use super::{FromLua, FromLuaErr, IntoLua};

impl FromLua for Integer {
    unsafe fn pop(l: *mut mlua_sys::lua_State) -> super::Result<Self> {
        let ty = unsafe { lua_type(l, -1) };
        if ty == LUA_TNONE {
            return Err(FromLuaErr::NotFound);
        }
        if ty != LUA_TNUMBER {
            return Err(FromLuaErr::IncorrectType);
        }

        // on 32 bit platforms tointeger returns an i32 so just cast it
        Ok(unsafe { lua_tointeger(l, -1) as Self })
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
    unsafe fn pop(l: *mut mlua_sys::lua_State) -> super::Result<Self> {
        let ty = unsafe { lua_type(l, -1) };
        if ty == LUA_TNONE {
            return Err(FromLuaErr::NotFound);
        }
        if ty != LUA_TNUMBER {
            return Err(FromLuaErr::IncorrectType);
        }

        Ok(unsafe { lua_tonumber(l, -1) })
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
