use crate::nvim_types::{Buffer, FromLua, Integer, ThinString};

pub struct OpenTermOnInputArgs<'a> {
    pub src: ThinString<'a>,
    pub term: Integer,
    pub buf: Buffer,
    pub data: ThinString<'a>,
}

impl <'a>OpenTermOnInputArgs<'a> {
    pub(crate) unsafe fn pop(l: *mut mlua_sys::lua_State) -> crate::nvim_types::lua::core::Result<OpenTermOnInputArgs<'a>> {
        Ok(unsafe {
            Self {
                src: ThinString::pop(l)?,
                term: Integer::pop(l)?,
                buf: Buffer::pop(l)?,
                data: ThinString::pop(l)?,
            }
        })
    }
}
