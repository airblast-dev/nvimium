use core::marker::PhantomData;

use mlua_sys::{LUA_REGISTRYINDEX, luaL_unref};
use thread_lock::get_lua_ptr;

use super::LuaRefT;

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LuaRef(LuaRefT, PhantomData<*mut LuaRefT>);

impl LuaRef {
    /// Initialize a new LuaRef with an key
    ///
    /// # Safety
    ///
    /// Passing an invalid key can result in UB in some cases the exact safety requirements depend
    /// on where it will passed.
    pub const unsafe fn new(key: LuaRefT) -> Self {
        Self(key, PhantomData)
    }

    /// Get the raw integer value of the [`LuaRef`]
    pub const fn as_int(&self) -> LuaRefT {
        self.0
    }
}

impl Drop for LuaRef {
    fn drop(&mut self) {
        unsafe { luaL_unref(get_lua_ptr().as_ptr(), LUA_REGISTRYINDEX, self.0) }
    }
}
