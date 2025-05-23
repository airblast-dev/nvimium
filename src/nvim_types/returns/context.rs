use std::mem::ManuallyDrop;

use crate::nvim_types::{Array, Dict, Object};

use super::utils::skip_drop_remove_keys;

#[derive(Debug)]
pub struct Context {
    pub regs: Array,
    pub jumps: Array,
    pub bufs: Array,
    pub gvars: Array,
    pub funcs: Array,
}

impl Context {
    pub fn from_c_func_ret(ctx: &mut Dict) -> Self {
        let [regs, jumps, bufs, gvars, funcs] =
            skip_drop_remove_keys(ctx, &["regs", "jumps", "bufs", "gvars", "funcs"], None)
                .unwrap()
                .map(|arr| {
                    if matches!(*arr, Object::Array(_)) {
                        Object::into_array(ManuallyDrop::into_inner(arr)).unwrap()
                    } else {
                        Array::default()
                    }
                });

        unsafe { ctx.0.set_len(0) };

        Self {
            regs,
            jumps,
            bufs,
            gvars,
            funcs,
        }
    }
}
