use macros::builder;

use crate::Boolean;

builder!(
    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct EchoOpts {
        pub verbose: Boolean,
    }
);
