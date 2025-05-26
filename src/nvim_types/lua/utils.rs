use std::error::Error;

use mlua_sys::{lua_State, lua_checkstack, lua_error};

use crate::{
    nvim_funcs::global::echo,
    nvim_types::{AsThinString, NvString, ThinString, func_types::echo::Echo, opts::echo::EchoOpts},
    plugin::IntoLua,
};

// whenever an error is returned from a callback this should be used
//
// this allows us to attempt multiple ways to print out an error to neovim
#[cold]
#[inline(never)]
pub(super) unsafe fn handle_callback_err_ret(l: *mut lua_State, err: &dyn Error) {
    use std::fmt::Write;
    let mut s = NvString::default();
    write!(s, "Error: {}", &err).unwrap();
    if let Err(echo_err) = echo(&Echo::message(s), true, EchoOpts::default().err(true)) {
        if echo(
            &Echo::message(echo_err.as_thinstr()),
            true,
            EchoOpts::default().err(true),
        )
        .is_err()
        {
            // TODO: while i doubt its possible, find a safer solution
            // since we should never reach this branch give a more helpful error message instead
            // of attempting to chug along
            debug_assert!(false, "Nvimium Internal Error: Unable to write error message to neovim (this likely a bug in nvimium)");
            unsafe {
                // lua_error performs a long jump over rust stack frames so
                // this is inherently unsound
                // (currently not UB but safety isn't guaranteed by spec)
                // ideally we never reach this branch
                if lua_checkstack(l, 1) == 0 {
                    panic!("Nvimium: Not enough stack space to push error message")
                }
                ThinString::from_null_terminated(
                    b"Nvimium: Error whilst writing error to neovim\0",
                )
                .push(l);
                lua_error(l);
            }
        }
    }
}
