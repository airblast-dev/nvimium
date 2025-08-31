use crate::{
    macros::{masked_builder::masked_builder, zeroed_default::zeroed_default},
    nvim_types::Boolean,
};

masked_builder!(
    #[repr(C)]
    pub struct GetCommandOpts {
        builtin: Boolean,
    }
);

zeroed_default!(GetCommandOpts);
