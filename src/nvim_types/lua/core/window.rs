use crate::nvim_types::{lua::LuaInteger, window::Window};

use super::IntoLua;

impl IntoLua for Window {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe { (self.as_int() as LuaInteger).push(l) };
    }
}
