use macros::masked_builder;

use crate::nvim_types::{Boolean, Integer, string::ThinString};

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
