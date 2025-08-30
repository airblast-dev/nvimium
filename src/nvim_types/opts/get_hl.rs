use crate::{
    macros::decl_derive::derive,
    nvim_types::{Boolean, Integer, string::ThinString},
};

derive!(
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    #[derive(Clone)]
    pub struct GetHlOpts<'a> {
        id: Integer,
        name: ThinString<'a>,
        link: Boolean,
        create: Boolean,
    }
);
