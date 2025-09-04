use crate::{
    macros::{masked_builder::masked_builder, zeroed_default::zeroed_default},
    nvim_types::{Boolean, Integer, string::ThinString, window::Window},
};
masked_builder!(
    #[derive(Clone)]
    #[repr(C)]
    pub struct EvalStatusLineOpts<'a> {
        winid: Window,
        maxwidth: Integer,
        #[builder(nv_str)]
        fillchar: ThinString<'a>,
        highlights: Boolean,
        use_winbar: Boolean,
        use_tabline: Boolean,
        use_statuscol_lnum: Integer,
    }
);

zeroed_default!(EvalStatusLineOpts<'_>);
