use crate::dictionary::{Dictionary, KeyValuePair};

use super::IntoLua;

impl IntoLua for Dictionary {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        for KeyValuePair { key, object } in self.iter() {
            todo!()
        }
    }
}
