use std::error::Error;

use crate::nvim_types::{
    IntoLua, TRACKED_ARENA, ThinString,
    lua::{set_callback_name, utils::handle_callback_err_ret},
};
use libc::c_int;
use thread_lock::{init_main_lua_ptr, scoped};

pub use mlua_sys::lua_State;
#[doc(hidden)]
pub use nvim_test::test_pkg;

/// The recommended way to define an entrypoint for a plugin
///
/// The first argument is `luaopen_<yourfunc>` where `<yourfunc>` is your plugins entrypoint and
/// the `<yourfunc>` identifier provided to the second argument.
///
/// # Example
/// ```no_run
/// use nvimium::{plugin, nvim_types::Error};
/// fn my_entry_point() -> Result<(), Error> {
///     // call neovim functions
///     Ok(())
/// }
///
/// plugin!(luaopen_my_entry_point, my_entry_point);
/// ```
///
/// # Panics
///
/// You may get confusing compile errors when incorrect identifiers are provided. Make sure
/// that the provided identifiers follow the specification mentioned above.
///
/// # Details
///
/// You might think this is fairly ugly, and you are correct. However this allows use to create
/// safe wrapper for a plugins entrypoint without using a proc macro, allowing for better compile times.
/// This could matter a lot if you intend to distribute your plugin as source and compile the code on the users end.
///
/// The first argument must be typed manually even though it always must be "luaopen_" +
/// "<your_func>", this is due to identifier concatenation not being stable (and it seems it never will
/// be). See [`core::concat_idents`] and its linked issue for more information.
#[macro_export]
macro_rules! plugin {
    ($open:ident, $ident:ident) => {
        #[cfg(test)]
        ::nvimium::nvim_test::test_pkg!();
        const _: () = const {
            const fn panic() -> ! {
                ::core::panic!();
            }
            const CORRECT_ENTRY: &'static [u8] = concat!("luaopen_", stringify!($ident)).as_bytes();
            const ENTRY_IDENT: &'static [u8] = stringify!($open).as_bytes();

            if CORRECT_ENTRY.len() != ENTRY_IDENT.len() {
                panic();
            }
            let mut i = 0;
            while i < CORRECT_ENTRY.len() {
                if CORRECT_ENTRY[i] != ENTRY_IDENT[i] || !CORRECT_ENTRY[i].is_ascii() {
                    panic();
                }
                i += 1;
            }
        };

        #[unsafe(no_mangle)]
        extern "C" fn $open(l: *mut $crate::plugin::lua_State) -> ::std::ffi::c_int {
            unsafe { $crate::plugin::open_plugin(l, $crate::gen_unique_ish_id!(), $ident) }
        }
    };
}

/// Initialize statics and create a scope for the plugin entrypoint
///
/// # Safety
///
/// This should never be called outside of the plugin entrypoint. Changing initialization related
/// values outside of the entrypoint will almost always result in a panic and may cause UB.
#[doc(hidden)]
#[inline]
pub unsafe fn open_plugin<Ret: IntoLua, Err: Sized + Error, F: Fn() -> Result<Ret, Err>>(
    l: *mut lua_State,
    cb_name: ThinString<'static>,
    open: F,
) -> c_int {
    unsafe {
        set_callback_name(cb_name.as_ptr() as *mut _);
        init_main_lua_ptr(l);
        scoped(
            |open| {
                let ret = open();
                // SAFETY: this is the entrypoint of our plugin, can never be called concurently
                // another mutable reference cannot exist at this point
                #[allow(static_mut_refs)]
                TRACKED_ARENA.reset_arena();
                match ret {
                    Ok(ret) => ret.push(l),
                    Err(err) => handle_callback_err_ret(l, &err as &dyn Error),
                }
            },
            open,
        );
    }

    // TODO: actually set ret vals
    0
}
