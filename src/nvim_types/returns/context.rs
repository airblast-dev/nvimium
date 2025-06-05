use std::{mem::ManuallyDrop, ops::Deref};

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
                .map(|arr| arr.deref().clone().into_array().unwrap());

        Self {
            regs,
            jumps,
            bufs,
            gvars,
            funcs,
        }
    }
}
