use crate::macros::{masked_builder::masked_builder, zeroed_default::zeroed_default};

masked_builder! {
    #[repr(C)]
    pub struct GetTextOpts {}
}

zeroed_default!(GetTextOpts);
