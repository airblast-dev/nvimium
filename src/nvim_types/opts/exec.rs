use macros::builder;

use crate::nvim_types::Boolean;

builder! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExecOpts {
        output: Boolean
    }
}
