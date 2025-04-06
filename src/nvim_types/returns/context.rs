use crate::nvim_types::{array::Array, dictionary::Dictionary};

#[derive(Debug)]
pub struct Context {
    pub regs: Array,
    pub jumps: Array,
    pub bufs: Array,
    pub gvars: Array,
    pub funcs: Array,
}

impl Context {
    pub fn from_c_func_ret(ctx: &mut Dictionary) -> Self {
        let regs = ctx
            .remove_skip_key_drop(c"regs")
            .unwrap()
            .into_array()
            .unwrap();
        let jumps = ctx
            .remove_skip_key_drop(c"jumps")
            .unwrap()
            .into_array()
            .unwrap();
        let bufs = ctx
            .remove_skip_key_drop(c"bufs")
            .unwrap()
            .into_array()
            .unwrap();
        let gvars = ctx
            .remove_skip_key_drop(c"gvars")
            .unwrap()
            .into_array()
            .unwrap();
        let funcs = ctx
            .remove_skip_key_drop(c"funcs")
            .unwrap()
            .into_array()
            .unwrap();

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
