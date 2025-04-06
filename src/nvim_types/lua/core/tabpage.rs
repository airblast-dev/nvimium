use crate::nvim_types::{lua::LuaInteger, tab_page::TabPage};

use super::IntoLua;

impl IntoLua for TabPage {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe { (self.as_int() as LuaInteger).push(l) };
    }
}
