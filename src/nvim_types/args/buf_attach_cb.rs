use mlua_sys::{LUA_TSTRING, lua_checkstack, lua_getstack, lua_gettop, lua_type};

use crate::nvim_types::{
    Buffer, Integer, ThinString,
    lua::{
        LuaInteger,
        core::{FromLua, FromLuaErr},
    },
};

pub struct BufOnLinesArgs<'a> {
    pub source: ThinString<'a>,
    pub bufnr: Buffer,
    pub changedtick: LuaInteger,
    pub first: LuaInteger,
    pub last_old: LuaInteger,
    pub last_new: LuaInteger,
    pub byte_count: LuaInteger,
    pub deleted_codepoints: Option<LuaInteger>,
    pub deleted_codeunits: Option<LuaInteger>,
}

impl<'a> crate::nvim_types::lua::core::FromLuaMany for BufOnLinesArgs<'a> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            let arg_count = lua_gettop(l);
            if !(arg_count == 7 || arg_count == 9) {
                return Err(FromLuaErr::NotFound);
            }

            let source = ThinString::get(l, 1, to_pop)?;
            let bufnr = <Buffer as FromLua>::get(l, 2, to_pop)?;
            let changedtick = <LuaInteger as FromLua>::get(l, 3, to_pop)?;
            let first = <LuaInteger as FromLua>::get(l, 4, to_pop)?;
            let last_old = <LuaInteger as FromLua>::get(l, 5, to_pop)?;
            let last_new = <LuaInteger as FromLua>::get(l, 6, to_pop)?;
            let byte_count = <LuaInteger as FromLua>::get(l, 7, to_pop)?;
            let (deleted_codepoints, deleted_codeunits) = if arg_count == 9 {
                (
                    Some(<LuaInteger as FromLua>::get(l, 8, to_pop)?),
                    Some(<LuaInteger as FromLua>::get(l, 9, to_pop)?),
                )
            } else {
                (None, None)
            };

            Ok(Self {
                source,
                bufnr,
                changedtick,
                first,
                last_old,
                last_new,
                byte_count,
                deleted_codepoints,
                deleted_codeunits,
            })
        }
    }
}

pub struct BufOnBytesArgs<'a> {
    pub source: ThinString<'a>,
    pub bufnr: Buffer,
    pub changedtick: Integer,
    pub start_row: Integer,
    pub start_col: Integer,
    pub start_byte: Integer,
    pub old_end_row: Integer,
    pub old_end_col: Integer,
    pub old_end_byte: Integer,
    pub new_end_row: Integer,
    pub new_end_col: Integer,
    pub new_end_byte: Integer,
}

impl<'a> crate::nvim_types::lua::core::FromLuaMany for BufOnBytesArgs<'a> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            let arg_count = lua_gettop(l);
            if arg_count != 12 {
                return Err(FromLuaErr::NotFound);
            }
            let source = ThinString::get(l, 1, to_pop)?;
            let bufnr = <Buffer as FromLua>::get(l, 2, to_pop)?;
            let changedtick = <Integer as FromLua>::get(l, 3, to_pop)?;
            let start_row = <Integer as FromLua>::get(l, 4, to_pop)?;
            let start_col = <Integer as FromLua>::get(l, 5, to_pop)?;
            let start_byte = <Integer as FromLua>::get(l, 6, to_pop)?;
            let old_end_row = <Integer as FromLua>::get(l, 7, to_pop)?;
            let old_end_col = <Integer as FromLua>::get(l, 8, to_pop)?;
            let old_end_byte = <Integer as FromLua>::get(l, 9, to_pop)?;
            let new_end_row = <Integer as FromLua>::get(l, 10, to_pop)?;
            let new_end_col = <Integer as FromLua>::get(l, 11, to_pop)?;
            let new_end_byte = <Integer as FromLua>::get(l, 12, to_pop)?;

            Ok(Self {
                source,
                bufnr,
                changedtick,
                start_row,
                start_col,
                start_byte,
                old_end_row,
                old_end_col,
                old_end_byte,
                new_end_row,
                new_end_col,
                new_end_byte,
            })
        }
    }
}

pub struct BufOnChangedTickArgs<'a> {
    pub source: ThinString<'a>,
    pub bufnr: Buffer,
    pub changedtick: Integer,
}

impl<'a> crate::nvim_types::lua::core::FromLuaMany for BufOnChangedTickArgs<'a> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            let arg_count = lua_gettop(l);
            if arg_count != 3 {
                return Err(FromLuaErr::NotFound);
            }

            let source = ThinString::get(l, 1, to_pop)?;
            let bufnr = <Buffer as FromLua>::get(l, 2, to_pop)?;
            let changedtick = <Integer as FromLua>::get(l, 3, to_pop)?;

            Ok(Self {
                source,
                bufnr,
                changedtick,
            })
        }
    }
}

pub struct BufOnDetach<'a> {
    pub source: ThinString<'a>,
    pub bufnr: Buffer,
}

impl<'a> crate::nvim_types::lua::core::FromLuaMany for BufOnDetach<'a> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            let arg_count = lua_gettop(l);
            if arg_count != 2 {
                return Err(FromLuaErr::NotFound);
            }

            let source = ThinString::get(l, 1, to_pop)?;
            let bufnr = <Buffer as FromLua>::get(l, 2, to_pop)?;

            Ok(Self { source, bufnr })
        }
    }
}

pub struct BufOnReload<'a> {
    pub source: ThinString<'a>,
    pub bufnr: Buffer,
}

impl<'a> crate::nvim_types::lua::core::FromLuaMany for BufOnReload<'a> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            let arg_count = lua_gettop(l);
            if arg_count != 2 {
                return Err(FromLuaErr::NotFound);
            }

            let source = ThinString::get(l, 1, to_pop)?;
            let bufnr = <Buffer as FromLua>::get(l, 2, to_pop)?;

            Ok(Self { source, bufnr })
        }
    }
}
