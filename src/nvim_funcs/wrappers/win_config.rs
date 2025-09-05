use thread_lock::call_check;

use crate::{
    macros::tri::tri_nc,
    nvim_funcs::c_funcs::win_config::nvim_open_win,
    nvim_types::{Boolean, Buffer, Error, Window, opts::win_opts::WinConfig},
};

pub fn open_win(buf: Buffer, enter: Boolean, config: &WinConfig) -> Result<Window, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_open_win(buf, enter, config, &raw mut err) };
    }
}

#[cfg(all(not(miri), feature = "testing"))]
mod tests {
    use crate::{
        self as nvimium,
        nvim_types::{
            Buffer,
            opts::win_opts::{Anchor, Border, Split, WinConfig},
        },
    };

    #[nvim_test::nvim_test]
    fn open_win_config() {
        let mut config = WinConfig::default();
        config
            .anchor(Anchor::NorthWest)
            .border(Border::Double)
            .row(2.)
            .col(6.)
            .focusable(true)
            .mouse(true)
            .split(Split::Above);
        super::open_win(Buffer::new(0), true, &config);
    }
}
