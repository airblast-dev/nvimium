use mlua_sys::{lua_checkstack, lua_pushnil};

use crate::nvim_types::Object;

use super::IntoLua;

impl IntoLua for Object {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        unsafe {
            match self {
                Self::Null => {
                    lua_checkstack(l, 1);
                    lua_pushnil(l);
                }
                Self::Bool(b) => b.push(l),
                Self::Integer(i) => i.push(l),
                Self::Float(f) => f.push(l),
                Self::Buffer(b) => b.push(l),
                Self::Dict(d) => d.push(l),
                Self::Array(a) => a.push(l),
                Self::String(s) => s.push(l),
                Self::Window(w) => w.push(l),
                Self::TabPage(t) => t.push(l),
                Self::LuaRef(lref) => lref.push(l),
            }
        }
    }
}
