use crate::{macros::{masked_builder::masked_builder, zeroed_default::zeroed_default}, nvim_types::Window};

masked_builder!(
    #[repr(C)]
    #[derive(Clone)]
    pub struct GetHlNsOpts {
        winid: Window,
    }
);

zeroed_default!(GetHlNsOpts);
