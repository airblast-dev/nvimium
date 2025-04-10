use std::mem::MaybeUninit;

use crate::nvim_types::{
    Arena, Array, Boolean, Channel, Dict, borrowed::Borrowed, error::Error, object::Object,
    opts::exec::ExecOpts, string::ThinString,
};

unsafe extern "C" {
    pub fn nvim_call_dict_function<'a>(
        dict: Object,
        func: ThinString<'a>,
        args: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_call_function<'a>(
        func: ThinString<'a>,
        args: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_command<'a>(command: ThinString<'a>, err: *mut Error);
    pub fn nvim_eval<'a>(
        eval: ThinString<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_exec2<'a>(
        chan: Channel,
        exec: ThinString<'a>,
        opts: *const ExecOpts,
        err: *mut Error,
    ) -> MaybeUninit<Dict>;
    pub fn nvim_parse_expression<'a>(
        expr: ThinString<'a>,
        flags: ThinString<'a>,
        highlight: Boolean,
        arena: *mut Arena,
        error: *mut Error,
    ) -> MaybeUninit<Dict>;
}
