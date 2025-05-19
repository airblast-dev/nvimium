use libc::c_int;
use mlua_sys::{LUA_TNONE, lua_checkstack, lua_pushboolean, lua_toboolean, lua_type};

use crate::nvim_types::Boolean;

use super::{FromLua, FromLuaErr, IntoLua};

impl FromLua for Boolean {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        index: c_int,
        to_pop: &mut i32,
    ) -> super::Result<Self> {
        if unsafe { lua_type(l, index) } == LUA_TNONE {
            return Err(FromLuaErr::NotFound);
        }
        *to_pop += 1;
        Ok(unsafe { lua_toboolean(l, index) } != 0)
    }
}

impl IntoLua for Boolean {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe {
            lua_checkstack(l, 1);
            lua_pushboolean(l, *self as _);
        }
    }
}
