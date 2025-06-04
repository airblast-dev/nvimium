use std::mem::MaybeUninit;

use mlua_sys::{
    LUA_TSTRING, LUA_TTABLE, lua_State, lua_checkstack, lua_getfield, lua_next, lua_objlen,
    lua_pop, lua_pushnil, lua_tolstring, lua_type,
};

use crate::{
    nvim_types::{
        Boolean, Integer, KVec, ThinString,
        lua::{
            LuaInteger,
            core::{FromLuaErr, FromLuaMany},
            utils::{get_table_bool_val, get_table_int_val, get_table_str_val},
        },
        opts::create_user_command::UserCommandNarg,
    },
    th,
};

pub struct UserCommandArgs<'a> {
    name: ThinString<'a>,
    args: ThinString<'a>,
    fargs: KVec<ThinString<'a>>,
    /// This is present after and including version 0.11.3 and is always some.
    /// Older versions will always have None.
    nargs: Option<UserCommandNarg>,
    bang: Boolean,
    line1: LuaInteger,
    line2: LuaInteger,
    range: LuaInteger,
    count: LuaInteger,
    reg: Option<ThinString<'a>>,
    mods: ThinString<'a>,
    smods: UserCommandSmods<'a>,
}

impl FromLuaMany for UserCommandArgs<'static> {
    unsafe fn get(
        l: *mut mlua_sys::lua_State,
        to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        *to_pop += 1;
        unsafe {
            if lua_checkstack(l, 2) == 0 {
                return Err(FromLuaErr::NotEnoughStackSpace);
            }

            if LUA_TTABLE != lua_type(l, -1) {
                return Err(FromLuaErr::IncorrectType);
            }

            let args = get_table_str_val(l, -1, c"args")?;
            let bang = get_table_bool_val(l, -1, c"bang")?;
            let count = get_table_int_val(l, -1, c"count")?;
            let fargs = 'fargs: {
                lua_getfield(l, -1, c"fargs".as_ptr());
                if LUA_TTABLE != lua_type(l, -1) {
                    lua_pop(l, 1);
                    return Err(FromLuaErr::IncorrectType);
                }
                let mut kv = KVec::with_capacity(lua_objlen(l, -1));
                if kv.capacity() == 0 {
                    lua_pop(l, 1);
                    break 'fargs kv;
                }
                let mut len: MaybeUninit<usize> = MaybeUninit::uninit();
                lua_pushnil(l);
                while lua_next(l, -2) != 0 {
                    if lua_type(l, -1) != LUA_TSTRING {
                        // remove the key, value and fargs table
                        lua_pop(l, 3);
                        return Err(FromLuaErr::IncorrectType);
                    }

                    let s = lua_tolstring(l, -1, (&raw mut len) as *mut usize);

                    // pop the string value but leave the key
                    lua_pop(l, 1);
                    kv.push(ThinString::new(len.assume_init(), s));
                }

                // once lua_next returns zero the only value we need to pop is the fargs table
                lua_pop(l, 1);

                kv
            };

            let line1 = get_table_int_val(l, -1, c"line1")?;
            let line2 = get_table_int_val(l, -1, c"line2")?;
            let mods = get_table_str_val(l, -1, c"mods")?;
            let name = get_table_str_val(l, -1, c"name")?;
            let range = get_table_int_val(l, -1, c"range")?;

            let reg = get_table_str_val(l, -1, c"reg").ok();
            let smods = {
                if lua_getfield(l, -1, c"smods".as_ptr()) != LUA_TTABLE {
                    lua_pop(l, 1);
                    return Err(FromLuaErr::IncorrectType);
                }

                UserCommandSmods::get(l, to_pop)?
            };

            let nargs = {
                get_table_str_val(l, -1, c"nargs").ok().map(|nargs_s| {
                    if th!("0") == nargs_s {
                        UserCommandNarg::ZERO
                    } else if th!("1") == nargs_s {
                        UserCommandNarg::ONE
                    } else if th!("?") == nargs_s {
                        UserCommandNarg::ZERO_OR_ONE
                    } else if th!("+") == nargs_s {
                        UserCommandNarg::ONE_OR_MORE
                    }
                    // "*" case
                    else {
                        UserCommandNarg::ZERO_OR_MORE
                    }
                })
            };

            Ok(Self {
                name,
                args,
                fargs,
                nargs,
                bang,
                line1,
                line2,
                range,
                count,
                reg,
                mods,
                smods,
            })
        }
    }
}

pub struct UserCommandSmods<'a> {
    browse: Boolean,
    confirm: Boolean,
    emsg_silent: Boolean,
    hide: Boolean,
    horizontal: Boolean,
    keepalt: Boolean,
    keepjumps: Boolean,
    keepmarks: Boolean,
    keeppatterns: Boolean,
    lockmarks: Boolean,
    noautocmd: Boolean,
    noswapfile: Boolean,
    sandbox: Boolean,
    silent: Boolean,
    split: ThinString<'a>,
    tab: LuaInteger,
    unsilent: Boolean,
    verbose: Integer,
    vertical: Boolean,
}

impl<'a> FromLuaMany for UserCommandSmods<'a> {
    unsafe fn get(
        l: *mut lua_State,
        _to_pop: &mut i32,
    ) -> crate::nvim_types::lua::core::Result<Self> {
        unsafe {
            if lua_checkstack(l, 1) == 0 {
                return Err(FromLuaErr::NotEnoughStackSpace);
            }

            let ret = Ok(UserCommandSmods {
                browse: get_table_bool_val(l, -1, c"browse")?,
                confirm: get_table_bool_val(l, -1, c"confirm")?,
                emsg_silent: get_table_bool_val(l, -1, c"emsg_silent")?,
                hide: get_table_bool_val(l, -1, c"hide")?,
                horizontal: get_table_bool_val(l, -1, c"horizontal")?,
                keepalt: get_table_bool_val(l, -1, c"keepalt")?,
                keepjumps: get_table_bool_val(l, -1, c"keepjumps")?,
                keepmarks: get_table_bool_val(l, -1, c"keepmarks")?,
                keeppatterns: get_table_bool_val(l, -1, c"keeppatterns")?,
                lockmarks: get_table_bool_val(l, -1, c"lockmarks")?,
                noautocmd: get_table_bool_val(l, -1, c"noautocmd")?,
                noswapfile: get_table_bool_val(l, -1, c"noswapfile")?,
                sandbox: get_table_bool_val(l, -1, c"sandbox")?,
                silent: get_table_bool_val(l, -1, c"silent")?,
                split: get_table_str_val(l, -1, c"split")?,
                tab: get_table_int_val(l, -1, c"tab")?,
                unsilent: get_table_bool_val(l, -1, c"unsilent")?,
                verbose: get_table_int_val(l, -1, c"verbose")?,
                vertical: get_table_bool_val(l, -1, c"vertical")?,
            });
            lua_pop(l, 1);
            ret
        }
    }
}
