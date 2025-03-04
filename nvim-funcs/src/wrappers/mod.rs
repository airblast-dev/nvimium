use macros::tri;
use nvim_types::{
    array::Array,
    buffer::Buffer,
    call_site::LUA_INTERNAL_CALL,
    error::Error,
    func_types::{feedkeys::FeedKeysMode, KeyMapMode},
    object::Object,
    opts::{echo::EchoOpts, eval_statusline::EvalStatusLineOpts},
    returns::eval_statusline::EvalStatusLineDict,
    string::{AsThinString, ThinString},
    Boolean,
};

use crate::c_funcs;

/// Create a new buffer
///
/// Returns [`Option::None`] if creating the buffer fails.
pub fn nvim_create_buf(listed: Boolean, scratch: Boolean) -> Result<Buffer, Error> {
    tri! {
            let mut err;
    unsafe { c_funcs::nvim_create_buf(listed, scratch, &mut err) }
        }
}

pub fn nvim_del_current_line() -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe {c_funcs::nvim_del_current_line(core::ptr::null_mut(), &mut err) },
    }
}

pub fn nvim_del_keymap<S: AsThinString>(map_mode: KeyMapMode, lhs: S) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_del_keymap(LUA_INTERNAL_CALL, map_mode, lhs.as_thinstr(), &mut err) }
    }
}

pub fn nvim_del_mark<S: AsThinString>(name: S) -> Result<Boolean, Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_del_mark(name.as_thinstr(), &mut err)
        }
    }
}

pub fn nvim_del_var<S: AsThinString>(var: S) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_del_var(var.as_thinstr(), &mut err);
        }
    }
}

pub fn nvim_echo<'a, S: AsThinString>(
    chunks: &'a Array,
    history: Boolean,
    opts: &'a EchoOpts,
) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_echo(chunks.into(), history, opts, &mut err);
        }
    }
}

pub fn nvim_err_write<S: AsThinString>(s: S) {
    unsafe { c_funcs::nvim_err_write(s.as_thinstr()) };
}

pub fn nvim_err_writeln<S: AsThinString>(s: S) {
    unsafe { c_funcs::nvim_err_writeln(s.as_thinstr()) };
}
pub fn nvim_eval_statusline<'a, S: AsThinString>(
    s: ThinString<'a>,
    opts: &EvalStatusLineOpts<'a>,
) -> Result<EvalStatusLineDict, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_eval_statusline(s.as_thinstr(), opts, core::ptr::null_mut(), &mut err) },
        Ok(ret) => {
            let ret = unsafe { ret.assume_init() };
            Ok(EvalStatusLineDict::from_c_func_ret(ret))
        }
    }
}

pub fn nvim_exec_lua<S: AsThinString>(code: S, args: &Array) -> Result<Object, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_exec_lua(code.as_thinstr(), args.into(), core::ptr::null_mut(), &mut err) },
        Ok(ret) => {
            Ok(unsafe{ret.assume_init()})
        }
    }
}

pub fn nvim_feedkeys(keys: ThinString, mode: &FeedKeysMode, escape_ks: Boolean) {
    unsafe {
        c_funcs::nvim_feedkeys(keys, mode.as_thinstr(), escape_ks);
    }
}
