use macros::masked_builder;

use crate::array::Array;

masked_builder! {
    #[repr(C)]
    pub struct ContextOpts {
        list: Array,
    }
}
