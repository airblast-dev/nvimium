use std::mem::MaybeUninit;

use crate::nvim_types::{
    Arena, Array, Boolean, Channel, Dict,
    borrowed::Borrowed,
    error::Error,
    object::{Object, ObjectRef},
    opts::exec::ExecOpts,
    string::ThinString,
};

unsafe extern "C" {
    pub fn nvim_call_dict_function(
        dict: ObjectRef<'_>,
        func: ThinString<'_>,
        args: Borrowed<'_, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_call_function<'a>(
        func: ThinString<'a>,
        args: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_command(command: ThinString<'_>, err: *mut Error);
    pub fn nvim_eval(
        eval: ThinString<'_>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_exec2(
        chan: Channel,
        exec: ThinString<'_>,
        opts: *const ExecOpts,
        err: *mut Error,
    ) -> MaybeUninit<Dict>;
    pub fn nvim_parse_expression(
        expr: ThinString<'_>,
        flags: ThinString<'_>,
        highlight: Boolean,
        arena: *mut Arena,
        error: *mut Error,
    ) -> MaybeUninit<Dict>;
}
