use macros::builder;

use crate::Boolean;

builder! {
    struct ExecOpts {
        output: Boolean
    }
}
