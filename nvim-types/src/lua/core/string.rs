use std::ffi::c_int;

use mlua_sys::{lua_tolstring, lua_type, LUA_TNONE, LUA_TSTRING};

use crate::string::{OwnedThinString, ThinString};

use super::{FromLua, FromLuaErr, Result};

#[doc(hidden)]
impl ThinString<'static> {
    /// Returns a [`ThinString`] with a static lifetime.
    ///
    /// Used to avoid an extra allocation in callbacks. The actual lifetime is how long this value
    /// ends up living on the lua stack allowing use until the end the of the callback.
    ///
    /// Only use internally as users are free to push and pop from the stack when using mlua inside
    /// callbacks
    unsafe fn pop(l: *mut mlua_sys::lua_State, idx: c_int) -> Result<Self> {
        unsafe {
            let ty = lua_type(l, idx);
            if LUA_TNONE == ty {
                return Err(FromLuaErr::NotFound);
            }
            if LUA_TSTRING != ty {
                return Err(FromLuaErr::IncorrectType);
            }
            let mut len = 0;
            let th: ThinString<'static> = ThinString::new(len, lua_tolstring(l, idx, &mut len));
            Ok(th)
        }
    }
}

impl FromLua for OwnedThinString {
    unsafe fn pop(l: *mut mlua_sys::lua_State, idx: c_int) -> Result<Self> {
        let th = unsafe { ThinString::pop(l, idx) }?;
        Ok(Self::from(th))
    }
}
