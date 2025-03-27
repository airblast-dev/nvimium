use mlua_sys::lua_toboolean;

use crate::Boolean;

use super::FromLua;

impl FromLua for Boolean {
    unsafe fn pop(l: *mut mlua_sys::lua_State) -> super::Result<Self> {
        Ok(unsafe { lua_toboolean(l, -1) } != 0)
    }
}
