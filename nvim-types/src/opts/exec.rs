use macros::builder;

use crate::Boolean;

builder! {
    #[derive(Default)]
    pub struct ExecOpts {
        output: Boolean
    }
}
