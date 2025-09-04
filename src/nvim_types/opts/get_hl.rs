use crate::{
    macros::{masked_builder::masked_builder, zeroed_default::zeroed_default},
    nvim_types::{Boolean, Integer, string::ThinString},
};

masked_builder!(
    #[repr(C)]
    #[derive(Clone)]
    pub struct GetHlOpts<'a> {
        id: Integer,
        #[builder(nv_str)]
        name: ThinString<'a>,
        link: Boolean,
        create: Boolean,
    }
);

zeroed_default!(GetHlOpts<'_>);
