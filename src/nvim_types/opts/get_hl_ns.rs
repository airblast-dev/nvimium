use crate::{macros::masked_builder::masked_builder, nvim_types::window::Window};

masked_builder!(
    #[repr(C)]
    #[derive(Clone)]
    pub struct GetHlNsOpts {
        winid: Window,
    }
);
