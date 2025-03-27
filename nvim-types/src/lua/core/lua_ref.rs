use mlua_sys::{LUA_REGISTRYINDEX, lua_rawgeti};

use crate::{lua::LuaInteger, lua_ref::LuaRef};

use super::IntoLua;

impl IntoLua for LuaRef {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe { lua_rawgeti(l, LUA_REGISTRYINDEX, self.as_int() as LuaInteger) };
    }
}
