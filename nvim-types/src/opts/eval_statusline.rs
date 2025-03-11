use macros::masked_builder;

use crate::{string::ThinString, window::Window, Boolean, Integer};

masked_builder!(
    #[derive(Clone, Debug)]
    #[repr(C)]
    pub struct EvalStatusLineOpts<'a> {
        winid: Window,
        max_width: Integer,
        fill_char: ThinString<'a>,
        highlights: Boolean,
        winbar: Boolean,
        tabline: Boolean,
        statuscol_lnum: Integer,
    }
);

impl Default for EvalStatusLineOpts<'_> {
    fn default() -> Self {
        // SAFETY: the mask blocks from any of the fields being read from neovim
        // and none the provided API's allow a user to read the values in rust making this safe
        unsafe { core::mem::zeroed() }
    }
}
