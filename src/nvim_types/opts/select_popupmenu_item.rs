use crate::macros::{masked_builder::masked_builder, zeroed_default::zeroed_default};

masked_builder! {
    #[repr(C)]
    #[derive(Clone)]
    pub struct SelectPopupMenuOpts {}
}

zeroed_default!(SelectPopupMenuOpts);
