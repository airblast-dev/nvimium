use macros::builder;

use crate::nvim_types::Boolean;

builder! {
    #[repr(C)]
    #[derive(Clone, Default)]
    pub struct ExecOpts {
        output: Boolean
    }
}
