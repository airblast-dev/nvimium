use crate::{
    macros::{masked_builder::masked_builder, zeroed_default::zeroed_default},
    nvim_types::{string::ThinString, Boolean, Integer},
};

masked_builder!(
    #[repr(C)]
    #[derive(Clone)]
    pub struct GetHlOpts<'a> {
        id: Integer,
        name: ThinString<'a>,
        link: Boolean,
        create: Boolean,
    }
);

zeroed_default!(GetHlOpts<'_>);
