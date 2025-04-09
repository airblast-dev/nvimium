use crate::masked_builder;

use crate::nvim_types::Array;

masked_builder! {
    #[repr(C)]
    pub struct ContextOpts {
        list: Array,
    }
}
