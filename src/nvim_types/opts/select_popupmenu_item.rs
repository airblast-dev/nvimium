use crate::macros::decl_derive::derive;

derive! {
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    #[derive(Clone)]
    pub struct SelectPopupMenuOpts {}
}
