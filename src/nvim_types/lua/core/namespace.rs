use crate::nvim_types::{lua::LuaInteger, namespace::NameSpace};

use super::IntoLua;

impl IntoLua for NameSpace {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe { (self.as_int() as LuaInteger).push(l) };
    }
}
