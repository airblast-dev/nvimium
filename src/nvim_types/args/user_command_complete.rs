use crate::nvim_types::{FromLua, Integer, ThinString};

pub struct UserCommandCompleteArgs<'a> {
    pub arg_lead: ThinString<'a>,
    pub cmd: ThinString<'a>,
    pub cursor_pos: Integer,
}

impl<'a> crate::nvim_types::lua::core::FromLuaMany for UserCommandCompleteArgs<'a> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            let arg_lead = ThinString::get(l, -3, to_pop)?;
            let cmd = ThinString::get(l, -2, to_pop)?;
            let cursor_pos = <Integer as FromLua>::get(l, -1, to_pop)?;

            Ok(Self {
                arg_lead,
                cmd,
                cursor_pos,
            })
        }
    }
}
