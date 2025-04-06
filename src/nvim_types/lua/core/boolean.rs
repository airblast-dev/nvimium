use mlua_sys::{lua_checkstack, lua_pushboolean, lua_toboolean};

use crate::nvim_types::Boolean;

use super::{FromLua, IntoLua};

impl FromLua for Boolean {
    unsafe fn pop(l: *mut mlua_sys::lua_State) -> super::Result<Self> {
        Ok(unsafe { lua_toboolean(l, -1) } != 0)
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
