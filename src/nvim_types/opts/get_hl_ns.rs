use macros::masked_builder;

use crate::nvim_types::window::Window;

masked_builder!(
    #[repr(C)]
    #[derive(Clone)]
    pub struct GetHlNsOpts {
        winid: Window,
    }
);
