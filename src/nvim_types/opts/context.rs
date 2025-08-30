use crate::{macros::decl_derive::derive, nvim_types::Array};

derive! {
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    pub struct ContextOpts {
        list: Array,
    }
}
