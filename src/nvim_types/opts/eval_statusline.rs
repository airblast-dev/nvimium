use macros::masked_builder;

use crate::nvim_types::{Boolean, Integer, string::ThinString, window::Window};

masked_builder!(
    #[derive(Clone)]
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
