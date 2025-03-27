use crate::{hl_group::HlGroupId, lua::LuaInteger};

use super::IntoLua;

impl IntoLua for HlGroupId {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe { (self.as_int() as LuaInteger).push(l) };
    }
}
