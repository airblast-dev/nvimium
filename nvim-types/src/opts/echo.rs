use macros::{builder, fast_default};

use crate::Boolean;

builder!(
    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct EchoOpts {
        pub verbose: Boolean,
    }
);

fast_default!(unsafe EchoOpts);
