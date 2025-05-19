use libc::{c_char, c_int};
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
    pub(crate) unsafe fn get(
        l: *mut mlua_sys::lua_State,
        index: c_int,
        to_pop: &mut i32,
    ) -> Result<Self> {
        unsafe {
            let ty = lua_type(l, index);
            if LUA_TNONE == ty {
                return Err(FromLuaErr::NotFound);
            }

            *to_pop += 1;
            if LUA_TSTRING != ty {
                return Err(FromLuaErr::IncorrectType);
            }
            let mut len = 0;
            let ptr = lua_tolstring(l, index, &mut len);
            let th: ThinString<'static> = ThinString::new(len, ptr);
            Ok(th)
        }
    }
}

impl FromLua for OwnedThinString {
    unsafe fn get(l: *mut mlua_sys::lua_State, index: c_int, to_pop: &mut i32) -> Result<Self> {
        let th = unsafe { ThinString::get(l, index, to_pop) }?;
        Ok(Self::from(th))
    }
}

impl<T: AsThinString> IntoLua for T {
    unsafe fn push(&self, l: *mut mlua_sys::lua_State) {
        let th = self.as_thinstr();
        unsafe { lua_pushlstring(l, th.as_ptr() as *const c_char, th.len()) };
    }
}
