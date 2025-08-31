use crate::{
    macros::{masked_builder::masked_builder, zeroed_default::zeroed_default},
    nvim_types::{string::ThinString, window::Window, Boolean, Integer},
};
masked_builder!(
    #[derive(Clone)]
    #[repr(C)]
    pub struct EvalStatusLineOpts<'a> {
        winid: Window,
        maxwidth: Integer,
        fillchar: ThinString<'a>,
        highlights: Boolean,
        use_winbar: Boolean,
        use_tabline: Boolean,
        use_statuscol_lnum: Integer,
    }
);

zeroed_default!(EvalStatusLineOpts<'_>);
