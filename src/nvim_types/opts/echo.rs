use macros::builder;

use crate::nvim_types::Boolean;

builder!(
    #[repr(C)]
    #[derive(Clone, Debug, Default)]
    pub struct EchoOpts {
        err: Boolean,
        verbose: Boolean,
    }
);
