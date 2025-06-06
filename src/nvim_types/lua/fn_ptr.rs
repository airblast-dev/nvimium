use std::error::Error;

use mlua_sys::{
    LUA_REGISTRYINDEX, lua_State, lua_checkstack, lua_pop, lua_pushcclosure, lua_pushlightuserdata,
    lua_tolightuserdata, lua_upvalueindex, luaL_ref,
};
use thread_lock::init_lua_ptr;

use crate::nvim_types::lua::{
    core::FromLuaMany,
    utils::{cb_ret_handle_arena, handle_callback_err_ret},
};

use super::IntoLua;

#[inline]
fn fn_callback<A: FromLuaMany, R: IntoLua, E: Error>() -> extern "C-unwind" fn(*mut lua_State) -> i32
{
    extern "C-unwind" fn callback<E: Error, A: FromLuaMany, R: IntoLua>(l: *mut lua_State) -> i32 {
        // before calling init in case a jump happens
        let fn_ptr =
            unsafe { lua_tolightuserdata(l, lua_upvalueindex(1)) } as *mut fn(A) -> Result<R, E>;
        unsafe { init_lua_ptr(l) };
        assert!(!fn_ptr.is_null());
        // TODO: handle return value
        unsafe {
            thread_lock::scoped_callback(
                |l| {
                    let mut to_pop = 0;
                    let arg = match A::get(l, &mut to_pop) {
                        Ok(arg) => arg,
                        Err(err) => {
                            handle_callback_err_ret(l, &err);
                            return 0;
                        }
                    };
                    let ret = (*fn_ptr)(arg);
                    lua_pop(l, to_pop);
                    match ret {
                        Ok(r) => 0,
                        Err(err) => {
                            handle_callback_err_ret(l, &err);
                            0
                        }
                    }
                },
                l,
                cb_ret_handle_arena,
            )
        }
    }

    callback::<E, A, R>
}

#[inline]
pub fn register<E: Error, A: FromLuaMany, R: IntoLua>(
    l: *mut lua_State,
    f: fn(A) -> Result<R, E>,
) -> i32 {
    let callback = fn_callback::<A, R, E>();
    unsafe {
        if lua_checkstack(l, 2) == 0 {
            panic!("not enough stack space to register function pointer");
        };
        lua_pushlightuserdata(l, f as *mut _);
        lua_pushcclosure(l, callback, 1);
        luaL_ref(l, LUA_REGISTRYINDEX)
    }
}
