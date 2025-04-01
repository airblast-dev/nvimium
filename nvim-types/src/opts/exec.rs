use macros::builder;

use crate::Boolean;

builder! {
    #[repr(C)]
    #[derive(Default)]
    pub struct ExecOpts {
        output: Boolean
    }
}
