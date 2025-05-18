use libc::c_char;
use mlua_sys::{LUA_TNONE, LUA_TSTRING, lua_pushlstring, lua_tolstring, lua_type};

use crate::nvim_types::{AsThinString, OwnedThinString, ThinString};

use super::{FromLua, FromLuaErr, IntoLua, Result};

#[doc(hidden)]
impl ThinString<'static> {
    /// Returns a [`ThinString`] with a static lifetime.
    ///
    /// Used to avoid an extra allocation in callbacks. The actual lifetime is how long this value
    /// ends up living on the lua stack allowing use until the end the of the callback.
    ///
    /// Only use internally as users are free to push and pop from the stack when using mlua inside
    /// callbacks
    pub(crate) unsafe fn pop(l: *mut mlua_sys::lua_State) -> Result<Self> {
        unsafe {
            let ty = lua_type(l, -1);
            if LUA_TNONE == ty {
                return Err(FromLuaErr::NotFound);
            }
            if LUA_TSTRING != ty {
                return Err(FromLuaErr::IncorrectType);
            }
            let mut len = 0;
            let th: ThinString<'static> = ThinString::new(len, lua_tolstring(l, -1, &mut len));
            Ok(th)
        }
    }
}

impl FromLua for OwnedThinString {
    unsafe fn pop(l: *mut mlua_sys::lua_State) -> Result<Self> {
        let th = unsafe { ThinString::pop(l) }?;
        Ok(Self::from(th))
    }
}

impl<T: AsThinString> IntoLua for T {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        let th = self.as_thinstr();
        unsafe { lua_pushlstring(l, th.as_ptr() as *const c_char, th.len()) };
    }
}
