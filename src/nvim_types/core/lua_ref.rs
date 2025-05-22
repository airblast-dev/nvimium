use core::marker::PhantomData;

use mlua_sys::{LUA_NOREF, LUA_REFNIL, LUA_REGISTRYINDEX, lua_rawgeti, luaL_ref, luaL_unref};
use thread_lock::{call_check, get_lua_ptr};

use crate::nvim_types::lua::LuaInteger;

use super::LuaRefT;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct LuaRef(LuaRefT, PhantomData<*mut LuaRefT>);

/// # Panics
///
/// Calls [`call_check`] so any panics caused by it also apply here.
impl Clone for LuaRef {
    fn clone(&self) -> Self {
        call_check();
        // checked if we have access above
        let l = get_lua_ptr().as_ptr();
        unsafe {
            lua_rawgeti(l, LUA_REGISTRYINDEX, self.as_int() as LuaInteger);
            Self::new(luaL_ref(l, LUA_REGISTRYINDEX))
        }
    }
}

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
    ///
    /// Passing this to [`LuaRef::new`] is unsound as it will cause double unref in Lua when the
    /// return [`LuaRef`] is dropped.
    /// While doing this itself is sound, another plugin or Neovim itself may experience undefined
    /// behavior if the correct checks are not performed. Best case doing so will result in hard to
    /// diagnose errors.
    pub const fn as_int(&self) -> LuaRefT {
        self.0
    }
}

impl Drop for LuaRef {
    fn drop(&mut self) {
        if self.0 != LUA_NOREF && self.0 != LUA_REFNIL {
            unsafe { luaL_unref(get_lua_ptr().as_ptr(), LUA_REGISTRYINDEX, self.0) }
        }
    }
}
