use macros::masked_builder;

use crate::window::Window;

masked_builder!(
    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct GetHlNsOpts {
        winid: Window,
    }
);
