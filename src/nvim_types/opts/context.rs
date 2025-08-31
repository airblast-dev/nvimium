use crate::{
    macros::{masked_builder::masked_builder, zeroed_default::zeroed_default},
    nvim_types::Array,
};

masked_builder!(
    #[repr(C)]
    pub struct ContextOpts {
        list: Array,
    }
);

zeroed_default!(ContextOpts);
