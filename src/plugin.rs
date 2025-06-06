#[doc(hidden)]
pub use crate::nvim_types::IntoLua;
#[doc(hidden)]
pub use thread_lock::scoped;

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
/// use nvimium::plugin;
/// fn my_entry_point() -> Result<(), &'static str> {
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
        extern "C" fn $open(lstate: *mut $crate::plugin::lua_State) -> ::std::ffi::c_int {
            unsafe { $crate::nvim_types::lua::set_callback_name($crate::gen_unique_ish_id!()) };
            let func: fn() -> _ = $ident;
            use ::core::result::Result;
            unsafe {
                $crate::thread_lock::init_main_lua_ptr(lstate);
                $crate::thread_lock::scoped(
                    |_| {
                        let ret = $ident();

                        match ret {
                            Ok(k) => $crate::nvim_types::lua::IntoLua::push(&k, lstate),
                            Err(err) => {
                                use std::fmt::Write;
                                use $crate::nvim_types::{NvString, ThinString};
                                use $crate::{
                                    nvim_funcs::global::echo,
                                    nvim_types::{func_types::echo::Echo, opts::echo::EchoOpts},
                                };
                                let mut msg = <NvString as ::std::default::Default>::default();
                                // TODO: add fallback messages
                                let _ = ::std::write!(&mut msg, "Nvimium Error: {}", err);
                                let _ = echo(
                                    &Echo::message(msg),
                                    true,
                                    <EchoOpts as ::std::default::Default>::default().err(true),
                                );
                            }
                        };
                        nvimium::nvim_types::arena::CALLBACK_ARENA.with_borrow_mut(|arena| {
                            *arena = nvimium::nvim_types::arena::Arena::EMPTY
                        });
                    },
                    (),
                );
            }
            1
        }
    };
}
