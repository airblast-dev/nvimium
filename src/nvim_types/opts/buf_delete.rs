use crate::{macros::decl_derive::derive, nvim_types::Boolean};

derive!{
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    pub struct BufDeleteOpts {
        force: Boolean,
        unload: Boolean,
    }
}
