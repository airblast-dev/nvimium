use macros::masked_builder;

use crate::{string::ThinString, Boolean, Integer};

masked_builder!(
    #[repr(C)]
    #[derive(Default, Debug)]
    pub struct GetHlOpts<'a> {
        name: ThinString<'a>,
        id: Integer,
        link: Boolean,
        create: Boolean,
    }
);
