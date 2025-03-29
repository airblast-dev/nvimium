use macros::builder;

use crate::Boolean;

builder!(
    #[repr(C)]
    #[derive(Clone, Debug, Default)]
    pub struct EchoOpts {
        err: Boolean,
        verbose: Boolean,
    }
);
