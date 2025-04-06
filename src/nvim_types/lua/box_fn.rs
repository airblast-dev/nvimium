use core::ffi::{CStr, c_char};
use std::sync::OnceLock;

use mlua_sys::{
    LUA_REGISTRYINDEX, lua_State, lua_checkstack, lua_newuserdata, lua_pop, lua_pushcclosure,
    lua_pushcfunction, lua_rawgeti, lua_setfield, lua_setmetatable, lua_touserdata,
    lua_upvalueindex, luaL_newmetatable, luaL_ref,
};

use super::FromLua;

trait TypeName {
    fn metatable_key(l: *mut lua_State) -> i32;
    fn type_name() -> &'static CStr;
}

fn type_name() -> *const c_char {
    static C: &CStr = c"Callback";
    C.as_ptr()
}

fn metatable_key(l: *mut lua_State) -> i32 {
    *KEY.get_or_init(|| unsafe {
        if lua_checkstack(l, 2) == 0 {
            panic!("not enough stack space to define Callback metatable")
        }
        if luaL_newmetatable(l, type_name()) == 0 {
            // luaL_newmetatable always pushes a value on the stack, cleanup before panicking
            lua_pop(l, 1);
            panic!("Callback metatable already exists");
        }

        lua_pushcfunction(l, drop_fn::<Box<dyn Fn(*mut lua_State)>>);
        lua_setfield(l, -2, c"__gc".as_ptr());

        luaL_ref(l, LUA_REGISTRYINDEX)
    })
}

static KEY: OnceLock<i32> = OnceLock::new();

pub fn register<F: 'static + Send + Sync + Fn(A) -> R, A: FromLua, R>(
    l: *mut lua_State,
    f: F,
) -> i32 {
    extern "C-unwind" fn call(l: *mut lua_State) -> i32 {
        let ud = unsafe { lua_touserdata(l, lua_upvalueindex(1)) };
        let cb: &dyn Fn(*mut lua_State) = unsafe {
            (ud as *mut Box<dyn Fn(*mut lua_State)>)
                .as_ref()
                .expect("registered closure's userdata pointer is null")
        };
        (cb)(l);
        0
    }
    unsafe {
        if lua_checkstack(l, 2) == 0 {
            panic!("not enough stack space to set Callback metatable");
        }

        // f must be moved or else it gets freed at the end of the scope
        let f: Box<dyn Fn(*mut lua_State)> = Box::new(move |l| {
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
    unsafe { ud.read() };
    0
}
