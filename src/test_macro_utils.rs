use std::{ffi::OsStr, path::Path, ffi::c_int};

use mlua_sys::lua_State;
use nvim_test::set_test_panic_hook;
use thread_lock::{init_main_lua_ptr, scoped};

use crate::{nvim_funcs::global::get_var, nvim_types::TRACKED_ARENA};

#[doc(hidden)]
pub unsafe fn test_body(l: *mut lua_State, test_call: fn(), exit_call: fn()) -> c_int {
    unsafe {
        init_main_lua_ptr(l);
        scoped(
            |_| {
                let test_out = get_var(c"NVIMIUM_PANIC_LOG_FILE")
                    .unwrap()
                    .into_string()
                    .unwrap();
                // we get and store the path as raw bytes
                let test_out = Path::new(OsStr::from_encoded_bytes_unchecked(
                    test_out.as_thinstr().as_slice(),
                ));
                set_test_panic_hook(test_out.to_path_buf());
                test_call();
                #[allow(static_mut_refs)]
                TRACKED_ARENA.reset_arena();
                exit_call();
            },
            (),
        );
    }

    0
}
