use crate::builder;

use crate::nvim_types::Boolean;

builder!(
    #[repr(C)]
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct EchoOpts {
        err: Boolean,
        verbose: Boolean,
    }
);
