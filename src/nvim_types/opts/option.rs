use crate::{
    macros::decl_derive::derive,
    nvim_types::{Buffer, ThinString, Window},
};

derive!(
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    pub struct OptionOpt<'a> {
        scope: ThinString<'a>,
        win: Window,
        buf: Buffer,
        filetype: ThinString<'a>,
    }
);
