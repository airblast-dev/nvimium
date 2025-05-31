use crate::{macros::masked_builder::masked_builder, nvim_types::{string::ThinString, window::Window, Boolean, Integer}};

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
