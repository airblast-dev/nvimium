use std::{error::Error, ffi::CStr, mem::MaybeUninit};

use libc::c_int;
use mlua_sys::{
    LUA_TBOOLEAN, LUA_TNUMBER, LUA_TSTRING, lua_State, lua_checkstack, lua_error, lua_getfield,
    lua_pop, lua_toboolean, lua_tointeger, lua_tolstring,
};

use crate::{
    nvim_funcs::global::echo,
    nvim_types::{
        AsThinString, Boolean, IntoLua, NvString, TRACKED_ARENA, ThinString,
        func_types::echo::Echo, opts::echo::EchoOpts,
    },
};

use super::{LuaInteger, core::FromLuaErr};

// whenever an error is returned from a callback this should be used
//
// this allows us to attempt multiple ways to print out an error to neovim
#[cold]
#[inline(never)]
pub(crate) unsafe fn handle_callback_err_ret(l: *mut lua_State, err: &dyn Error) {
    let mut s = NvString::default();
    write!(s, "Error: {}", &err).unwrap();
    if let Err(echo_err) = echo(&Echo::message(s), true, EchoOpts::default().err(true))
        && echo(
            &Echo::message(echo_err.as_thinstr()),
            true,
            EchoOpts::default().err(true),
        )
        .is_err()
    {
        // TODO: while i doubt its possible, find a safer solution
        // since we should never reach this branch give a more helpful error message instead
        // of attempting to chug along
        debug_assert!(
            false,
            "Nvimium Internal Error: Unable to write error message to neovim (this likely a bug in nvimium)"
        );
        unsafe {
            // lua_error performs a long jump over rust stack frames so
            // this is inherently unsound
            // (currently not UB but safety isn't guaranteed by spec)
            // ideally we never reach this branch
            if lua_checkstack(l, 1) == 0 {
                panic!("Nvimium: Not enough stack space to push error message")
            }
            ThinString::from_null_terminated(b"Nvimium: Error whilst writing error to neovim\0")
                .push(l);
            lua_error(l);
        }
    }
}

#[cold]
#[inline(never)]
fn incorrect_type() {}

/// Returns error if TYPE is incorrect and pops the incorrect value.
///
/// If correct pushes the TYPE at the top of the stack
pub(crate) unsafe fn get_table_val(
    l: *mut lua_State,
    i: c_int,
    field: &CStr,
    type_t: c_int,
) -> Result<(), FromLuaErr> {
    unsafe {
        if lua_getfield(l, i, field.as_ptr()) != type_t {
            incorrect_type();
            lua_pop(l, 1);
            return Err(FromLuaErr::IncorrectType);
        }

        Ok(())
    }
}

/// The return value is guaranteed that it lives as long as the table at `i` is on the stack
pub(crate) unsafe fn get_table_str_val(
    l: *mut lua_State,
    i: c_int,
    field: &CStr,
) -> Result<ThinString<'static>, FromLuaErr> {
    unsafe {
        #[allow(clippy::question_mark)]
        if let Err(err) = get_table_val(l, i, field, LUA_TSTRING) {
            return Err(err);
        };

        let mut len: MaybeUninit<usize> = MaybeUninit::uninit();
        let s = lua_tolstring(l, -1, (&raw mut len) as _);

        // popping this does not invalidate `s` as its still referenced by the table
        lua_pop(l, 1);
        Ok(ThinString::new(len.assume_init(), s))
    }
}

pub(crate) unsafe fn get_table_bool_val(
    l: *mut lua_State,
    i: c_int,
    field: &CStr,
) -> Result<Boolean, FromLuaErr> {
    unsafe {
        #[allow(clippy::question_mark)]
        if let Err(err) = get_table_val(l, i, field, LUA_TBOOLEAN) {
            return Err(err);
        }
        let ret = lua_toboolean(l, -1) != 0;
        lua_pop(l, 1);
        Ok(ret)
    }
}

pub(crate) unsafe fn get_table_int_val(
    l: *mut lua_State,
    i: c_int,
    field: &CStr,
) -> Result<LuaInteger, FromLuaErr> {
    unsafe {
        #[allow(clippy::question_mark)]
        if let Err(err) = get_table_val(l, i, field, LUA_TNUMBER) {
            return Err(err);
        }
        let ret = lua_tointeger(l, -1);
        lua_pop(l, 1);
        Ok(ret)
    }
}

pub(crate) unsafe fn cb_ret_handle_arena(was_active: bool) {
    if !was_active {
        // if was active is false this is the top level call, no mutable references can exist.
        #[allow(static_mut_refs)]
        unsafe {
            TRACKED_ARENA.reset_arena()
        };
    }
    unsafe { (&raw mut TRACKED_ARENA.is_nested).write(was_active) };
}

pub(crate) unsafe fn cb_entry_set_arena_flag(was_active: bool) {
    unsafe { (&raw mut TRACKED_ARENA.is_nested).write(was_active) };
}
