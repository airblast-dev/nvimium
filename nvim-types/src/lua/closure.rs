use mlua_sys::{
    luaL_ref, lua_State, lua_checkstack, lua_createtable, lua_newuserdata, lua_pushcclosure,
    lua_pushcfunction, lua_setfield, lua_setmetatable, lua_touserdata, lua_upvalueindex,
    LUA_REGISTRYINDEX,
};

use super::FromLua;

#[inline]
fn closure_drop<F: Fn(A) -> R + Send + Sync + Unpin, A: FromLua, R>() -> (
    extern "C-unwind" fn(*mut lua_State) -> i32,
    extern "C-unwind" fn(*mut lua_State) -> i32,
) {
    extern "C-unwind" fn callback<F: Fn(A) -> R, A: FromLua, R>(l: *mut lua_State) -> i32 {
        let ud = unsafe { lua_touserdata(l, lua_upvalueindex(1)) } as *mut F;
        unsafe { (ud.as_ref().unwrap())(core::mem::zeroed()) };
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
pub fn register<F: 'static + Fn(A) -> R + Send + Sync + Unpin, A: FromLua, R>(
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
