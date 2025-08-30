use crate::macros::decl_derive::derive;

derive!{
    derive(zeroed_default, masked_builder);
    #[repr(C)]
    pub struct SetMarkOpts {}
}
