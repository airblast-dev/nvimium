use mlua_sys::{
    LUA_REGISTRYINDEX, lua_State, lua_checkstack, lua_createtable, lua_newuserdata, lua_pop,
    lua_pushcclosure, lua_pushcfunction, lua_setfield, lua_setmetatable, lua_touserdata,
    lua_upvalueindex, luaL_ref,
};
use thread_lock::init_lua_ptr;

use crate::nvim_types::lua::core::FromLuaMany;

use super::{FromLua, IntoLua};

#[inline]
fn closure_drop<F: Fn(A) -> R + Unpin, A: FromLua, R>() -> (
    extern "C-unwind" fn(*mut lua_State) -> i32,
    extern "C-unwind" fn(*mut lua_State) -> i32,
) {
    extern "C-unwind" fn callback<F: Fn(A) -> R, A: FromLuaMany, R>(l: *mut lua_State) -> i32 {
        // before calling init in case a jump happens
        let ud = unsafe { lua_touserdata(l, lua_upvalueindex(1)) } as *mut F;
        unsafe { init_lua_ptr(l) };
        unsafe {
            thread_lock::scoped(
                |_| {
                    let mut to_pop = 0;
                    let ret = (ud.as_ref().unwrap())(A::get(l, &mut to_pop).unwrap());
                    lua_pop(l, to_pop);
                    ret
                },
                (),
            )
        };
        0
    }
    extern "C-unwind" fn drop_fn<T: Fn(A) -> R, A: FromLua, R>(l: *mut lua_State) -> i32 {
        let ud = unsafe { lua_touserdata(l, 1) } as *mut T;
        if !ud.is_null() {
            unsafe { ud.read() };
        }
        0
    }

    (callback::<F, A, R>, drop_fn::<F, A, R>)
}

#[inline]
pub fn register<F: 'static + Fn(A) -> R + Unpin, A: FromLua, R: IntoLua>(
    l: *mut lua_State,
    f: F,
) -> i32 {
    let (callback, drop_fn) = closure_drop::<F, A, R>();
    unsafe {
        let ud = lua_newuserdata(l, size_of::<F>()) as *mut F;
        ud.write(f);
        if lua_checkstack(l, 3) == 0 {
            panic!("not enough stack space to push rust closure");
        }
        lua_createtable(l, 0, 1);
        lua_pushcfunction(l, drop_fn);
        lua_setfield(l, -2, c"__gc".as_ptr());

        lua_setmetatable(l, -1);
        lua_pushcclosure(l, callback, 1);
        luaL_ref(l, LUA_REGISTRYINDEX)
    }
}
