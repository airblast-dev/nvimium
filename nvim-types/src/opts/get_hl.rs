use macros::masked_builder;

use crate::{Boolean, Integer, string::ThinString};

masked_builder!(
    #[repr(C)]
    pub struct GetHlOpts<'a> {
        id: Integer,
        name: ThinString<'a>,
        link: Boolean,
        create: Boolean,
    }
);
