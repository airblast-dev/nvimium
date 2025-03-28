use mlua_sys::{LUA_NOREF, LUA_REGISTRYINDEX, luaL_unref};

use super::LuaRefT;

#[repr(transparent)]
#[derive(Debug)]
pub struct LuaRef(LuaRefT);

impl LuaRef {
    /// Initialize a new LuaRef with an key
    ///
    /// # Safety
    ///
    /// Passing an invalid key can result in UB in some cases.
    pub unsafe fn new(key: LuaRefT) -> Self {
        Self(key)
    }
    pub fn as_int(&self) -> LuaRefT {
        self.0
    }
}

impl Drop for LuaRef {
    fn drop(&mut self) {
        if self.0 != LUA_NOREF {
            unsafe { luaL_unref(todo!("replace with thread local lua ref"), LUA_REGISTRYINDEX, self.as_int()) };
        }
    }
}
