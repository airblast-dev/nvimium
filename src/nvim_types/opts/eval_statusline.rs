use crate::{
    macros::decl_derive::derive,
    nvim_types::{Boolean, Integer, string::ThinString, window::Window},
};

derive!(
    derive(masked_builder, zeroed_default);
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
