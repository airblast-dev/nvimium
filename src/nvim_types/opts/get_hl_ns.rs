use crate::{macros::decl_derive::derive, nvim_types::Window};

derive!(
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    #[derive(Clone)]
    pub struct GetHlNsOpts {
        winid: Window,
    }
);
