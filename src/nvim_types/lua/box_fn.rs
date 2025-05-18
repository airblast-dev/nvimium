use core::ffi::c_char;
use std::{
    ffi::CStr,
    sync::{
        OnceLock,
        atomic::{AtomicPtr, Ordering},
    },
};

use mlua_sys::{
    LUA_REGISTRYINDEX, lua_State, lua_checkstack, lua_newuserdata, lua_pop, lua_pushcclosure,
    lua_pushcfunction, lua_rawgeti, lua_setfield, lua_setmetatable, lua_touserdata,
    lua_upvalueindex, luaL_newmetatable, luaL_ref,
};
use rand::{SeedableRng, distr::Distribution, rngs::SmallRng};
use thread_lock::{init_lua_ptr, unlock};

use crate::nvim_types::String;

use super::FromLua;

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
            let mut s = String::with_capacity(FALLBACK_TYPE_NAME.count_bytes() + RAND_CHAR_COUNT);
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

pub fn register<F: 'static + Fn(A) -> R, A: FromLua, R>(l: *mut lua_State, f: F) -> i32 {
    extern "C-unwind" fn call(l: *mut lua_State) -> i32 {
        // before calling init in case a jump happens
        let ud = unsafe { lua_touserdata(l, lua_upvalueindex(1)) };
        unsafe { init_lua_ptr(l) };
        let cb: &dyn Fn(*mut lua_State) = unsafe {
            (ud as *mut Box<dyn Fn(*mut lua_State)>)
                .as_ref()
                .expect("registered closure's userdata pointer is null")
        };
        unsafe {
            thread_lock::scoped(
                move |_| {
                    (cb)(l);
                },
                (),
            )
        };
        0
    }
    unsafe {
        if lua_checkstack(l, 2) == 0 {
            panic!("not enough stack space to set Callback metatable");
        }

        // f must be moved or else it gets freed at the end of the scope
        let f: Box<dyn Fn(*mut lua_State)> = Box::new(move |l| {
            // instead of double boxing, get the args here
            let arg = A::pop(l).unwrap();
            f(arg);
        });

        let ud = lua_newuserdata(l, size_of::<Box<dyn Fn(*mut lua_State)>>())
            as *mut Box<dyn Fn(*mut lua_State)>;
        ud.write(f);
        let mt_key = metatable_key(l);
        lua_rawgeti(l, LUA_REGISTRYINDEX, mt_key.into());
        lua_setmetatable(l, -2);

        lua_pushcclosure(l, call, 1);
        luaL_ref(l, LUA_REGISTRYINDEX)
    }
}

extern "C-unwind" fn drop_fn<D: Unpin>(l: *mut lua_State) -> i32 {
    let ud = unsafe { lua_touserdata(l, -1) } as *mut D;
    unsafe { ud.drop_in_place() };
    0
}
