use crate::{macros::decl_derive::derive, nvim_types::Boolean};

derive!(
    derive(zeroed_default, builder);
    #[repr(C)]
    pub struct GetCommandOpts {
        builtin: Boolean,
    }
);
