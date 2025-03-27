#[doc(hidden)]
pub use mlua_sys::lua_State;
#[doc(hidden)]
pub use nvim_types::IntoLua;
#[doc(hidden)]
pub use thread_lock::scoped;

/// The recommended way to define an entrypoint for a plugin
///
/// The first argument is `luaopen_<yourfunc>` where `<yourfunc>` is your plugins entrypoint and
/// the `<yourfunc>` identifier provided to the second argument.
///
/// # Example
/// ```
/// # use plugin::plugin;
/// fn my_entry_point() {
///     // call neovim functions
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
        pub const CDYLIB_TEST_PATH: ::std::sync::LazyLock<core::path::PathBuf> =
            ::std::sync::LazyLock(::nvimium::test_cdylib::build_current_project);
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
        extern "C" fn $open(lstate: *mut $crate::lua_State) -> usize {
            let func: fn() -> _ = $ident;
            let ret = unsafe { $crate::scoped(|_| $ident(), ()) };
            unsafe { $crate::IntoLua::push(&ret, lstate) };
            1
        }
    };
}
