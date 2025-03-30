use mlua_sys::{lua_checkstack, lua_createtable, lua_rawseti};

use crate::array::Array;

use super::IntoLua;

impl IntoLua for Array {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe { lua_checkstack(l, 2) };
        unsafe {
            lua_createtable(l, self.0.len().try_into().unwrap(), 0);
        }
        for (obj, i) in self.0.iter().zip(1..self.0.len()) {
            unsafe {
                obj.push(l);
                lua_rawseti(l, -2, i.try_into().unwrap());
            }
        }
    }
}
