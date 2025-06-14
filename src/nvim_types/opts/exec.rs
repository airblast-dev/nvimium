use crate::{macros::builder, nvim_types::Boolean};

builder! {
    #[repr(C)]
    #[derive(Clone, Default)]
    pub struct ExecOpts {
        output: Boolean
    }
}
