use crate::{macros::masked_builder::masked_builder, nvim_types::Array};

masked_builder! {
    #[repr(C)]
    pub struct ContextOpts {
        list: Array,
    }
}
