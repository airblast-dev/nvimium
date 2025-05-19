use mlua_sys::{lua_pop, lua_tolstring, lua_type, lua_typename};

use crate::nvim_types::{Buffer, FromLua, Integer, String, ThinString};

pub struct OpenTermOnInputArgs<'a> {
    pub src: ThinString<'a>,
    pub term: Integer,
    pub buf: Buffer,
    pub data: ThinString<'a>,
}

impl<'a> OpenTermOnInputArgs<'a> {
    pub(crate) unsafe fn pop(
        l: *mut mlua_sys::lua_State,
    ) -> crate::nvim_types::lua::core::Result<OpenTermOnInputArgs<'a>> {
        unsafe {
            let data = ThinString::pop(l)?;
            lua_pop(l, 1);
            let buf = Buffer::pop(l)?;
            lua_pop(l, 1);
            let term = Integer::pop(l)?;
            lua_pop(l, 1);
            let src = ThinString::pop(l)?;
            lua_pop(l, 1);
            Ok(Self {
                src,
                term,
                buf,
                data,
            })
        }
    }
}
