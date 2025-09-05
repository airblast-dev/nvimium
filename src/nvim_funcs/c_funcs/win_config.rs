use std::mem::MaybeUninit;

use crate::nvim_types::{Boolean, Buffer, Error, Window, opts::win_opts::WinConfig};

unsafe extern "C" {
    pub fn nvim_open_win(
        buf: Buffer,
        enter: Boolean,
        config: *const WinConfig,
        err: *mut Error,
    ) -> MaybeUninit<Window>;
}
