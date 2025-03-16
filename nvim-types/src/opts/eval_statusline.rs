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
