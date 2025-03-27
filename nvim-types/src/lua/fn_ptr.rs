use mlua_sys::{
    LUA_REGISTRYINDEX, lua_State, lua_checkstack, lua_pushcclosure, lua_pushlightuserdata,
    lua_tolightuserdata, lua_upvalueindex, luaL_ref,
};

use super::{FromLuaMulti, IntoLuaMulti};

#[inline]
fn fn_callback<A: FromLuaMulti, R: IntoLuaMulti>() -> extern "C-unwind" fn(*mut lua_State) -> i32 {
    extern "C-unwind" fn callback<A: FromLuaMulti, R: IntoLuaMulti>(l: *mut lua_State) -> i32 {
        let fn_ptr = unsafe { lua_tolightuserdata(l, lua_upvalueindex(1)) } as *mut fn(A) -> R;
        assert!(!fn_ptr.is_null());
        unsafe {
            (*fn_ptr)(A::pop(l).unwrap());
        };
        0
    }

    callback::<A, R>
}

#[inline]
pub fn register<A: FromLuaMulti, R: IntoLuaMulti>(l: *mut lua_State, f: fn(A) -> R) -> i32 {
    let callback = fn_callback::<A, R>();
    unsafe {
        if lua_checkstack(l, 2) == 0 {
            panic!("not enough stack space to register function pointer");
        };
        lua_pushlightuserdata(l, f as *mut _);
        lua_pushcclosure(l, callback, 1);
        luaL_ref(l, LUA_REGISTRYINDEX)
    }
}
