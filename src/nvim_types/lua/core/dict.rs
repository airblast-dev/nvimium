use libc::c_char;
use mlua_sys::{lua_checkstack, lua_createtable, lua_setfield};

use crate::nvim_types::{Dict, KeyValuePair};

use super::IntoLua;

impl IntoLua for Dict {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe {
            lua_checkstack(l, 2);
            lua_createtable(l, 0, self.len().try_into().unwrap())
        };
        for KeyValuePair { key, object } in self.iter() {
            unsafe {
                object.push(l);
                // we intentionally dont use OwnedThinString's IntoLua implementation as we can
                // lessen the stack requirements by using lua_setfield
                lua_setfield(l, -2, key.as_thinstr().as_ptr() as *const c_char);
            }
        }
    }
}
