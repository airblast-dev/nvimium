use core::ffi::c_char;
use std::{
    error::Error,
    ffi::CStr,
    sync::{
        OnceLock,
        atomic::{AtomicPtr, Ordering},
    },
};

use libc::c_int;
use mlua_sys::{
    LUA_REGISTRYINDEX, lua_State, lua_checkstack, lua_newuserdata, lua_pop, lua_pushcclosure,
    lua_pushcfunction, lua_pushlightuserdata, lua_rawgeti, lua_setfield, lua_setmetatable,
    lua_tolightuserdata, lua_touserdata, lua_upvalueindex, luaL_newmetatable, luaL_ref,
};
use rand::{SeedableRng, distr::Distribution, rngs::SmallRng};
use thread_lock::init_lua_ptr;

use crate::nvim_types::{NvString, lua::utils::handle_callback_err_ret};

use super::core::FromLuaMany;

static FALLBACK_TYPE_NAME: &CStr = c"NVIMIUM FALLBACK CALLBACK ID";
static TYPE_NAME: AtomicPtr<c_char> = AtomicPtr::new(FALLBACK_TYPE_NAME.as_ptr() as *mut c_char);

// TODO: return results in the functions below.

/// # SAFETY
///
/// Must point to a null terminated string.
pub unsafe fn set_callback_name(cstr: *mut c_char) {
    TYPE_NAME.store(cstr, Ordering::Relaxed);
}
fn type_name() -> *const c_char {
    TYPE_NAME.load(Ordering::Relaxed)
}

fn metatable_key(l: *mut lua_State) -> i32 {
    *KEY.get_or_init(|| unsafe {
        if lua_checkstack(l, 2) == 0 {
            panic!("not enough stack space to define Callback metatable")
        }

        let mut key_status = luaL_newmetatable(l, type_name());
        if key_status == 0 {
            lua_pop(l, 1);
        }

        let mut loop_count = 0;
        const RAND_CHAR_COUNT: usize = 128;

        // we have no way to control who inserts what metatable so no solution will technically be
        // flawless. This is a best effort fallback
        #[cold]
        #[inline(never)]
        fn cold() {}
        while key_status == 0 && loop_count < 100 {
            cold();
            let dist = rand::distr::Uniform::new(32_u8, 128_u8).unwrap();
            let c_char_iter =
                dist.sample_iter(SmallRng::seed_from_u64(&raw const TYPE_NAME as u64));
            let mut s = NvString::with_capacity(FALLBACK_TYPE_NAME.count_bytes() + RAND_CHAR_COUNT);
            for c in c_char_iter.take(RAND_CHAR_COUNT) {
                s.push([c]);
            }

            key_status = luaL_newmetatable(l, s.as_ptr() as *const c_char);
            if key_status == 0 {
                TYPE_NAME.store(s.as_ptr() as *mut c_char, Ordering::Relaxed);
            }
            lua_pop(l, 1);
            loop_count += 1;
        }

        if loop_count >= 100 {
            // Its practically impossible to reach this branch but better than having UB that is
            // near impossible to detect.
            libc::abort();
        }

        lua_pushcfunction(l, drop_fn::<Box<dyn Fn(*mut lua_State)>>);
        lua_setfield(l, -2, c"__gc".as_ptr());

        luaL_ref(l, LUA_REGISTRYINDEX)
    })
}

static KEY: OnceLock<i32> = OnceLock::new();

pub fn register<E: Error, F: 'static + Fn(A) -> Result<R, E>, A: FromLuaMany, R>(
    l: *mut lua_State,
    f: F,
) -> i32 {
    extern "C-unwind" fn call(l: *mut lua_State) -> i32 {
        // before calling init in case a jump happens
        unsafe {
            // sanity check: check if our user data is actually the one associated with the closure
            // by comparing the associated `typename`
            assert!(core::ptr::addr_eq(
                lua_tolightuserdata(l, lua_upvalueindex(2)),
                type_name()
            ));
            let ud = lua_touserdata(l, lua_upvalueindex(1));
            init_lua_ptr(l);
            let cb: &dyn Fn(*mut lua_State) -> c_int =
                (ud as *mut Box<dyn Fn(*mut lua_State) -> c_int>)
                    .as_ref()
                    .expect("registered closure's userdata pointer is null");
            thread_lock::scoped(cb, l)
        }
    }
    unsafe {
        if lua_checkstack(l, 2) == 0 {
            panic!("not enough stack space to set Callback metatable");
        }

        // f must be moved or else it gets freed at the end of the scope
        let f: Box<dyn Fn(*mut lua_State) -> c_int> = Box::new(move |l| {
            let mut to_pop = 0;
            let arg = match A::get(l, &mut to_pop) {
                Ok(arg) => arg,
                Err(err) => {
                    handle_callback_err_ret(l, &err);
                    return 0;
                }
            };
            let ret = f(arg);
            lua_pop(l, to_pop);
            match ret {
                Ok(r) => 0,
                Err(err) => {
                    handle_callback_err_ret(l, &err);
                    0
                }
            }
        });

        let ud = lua_newuserdata(l, size_of::<Box<dyn Fn(*mut lua_State) -> c_int>>())
            as *mut Box<dyn Fn(*mut lua_State) -> c_int>;
        ud.write(f);
        let mt_key = metatable_key(l);
        lua_rawgeti(l, LUA_REGISTRYINDEX, mt_key.into());
        lua_setmetatable(l, -2);

        // we associate the function identifier with the closure
        // later we check if the pointer is the same in the function call
        lua_pushlightuserdata(l, type_name() as *mut _);

        lua_pushcclosure(l, call, 2);
        luaL_ref(l, LUA_REGISTRYINDEX)
    }
}

extern "C-unwind" fn drop_fn<D: Unpin>(l: *mut lua_State) -> i32 {
    let ud = unsafe { lua_touserdata(l, -1) } as *mut D;
    debug_assert!(!ud.is_null());
    if !ud.is_null() {
        unsafe { ud.drop_in_place() };
    }
    0
}
