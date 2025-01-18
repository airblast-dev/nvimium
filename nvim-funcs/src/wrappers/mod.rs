use macros::tri;
use nvim_types::{
    buffer::Buffer, call_site::LUA_INTERNAL_CALL, error::Error, func_types::KeyMapMode,
    string::ThinString, Boolean,
};

use crate::c_funcs;

/// Create a new buffer
///
/// Returns [`Option::None`] if creating the buffer fails.
pub fn nvim_create_buf(listed: Boolean, scratch: Boolean) -> Option<Buffer> {
    let buf = unsafe { c_funcs::nvim_create_buf(listed, scratch) };
    if buf.as_int() != 0 {
        Some(buf)
    } else {
        None
    }
}

pub fn nvim_del_current_line() -> Result<(), Error> {
    tri! {
        err,
        unsafe {c_funcs::nvim_del_current_line(core::ptr::null_mut(), &mut err) },
    }
}

pub fn nvim_del_keymap(map_mode: KeyMapMode, lhs: ThinString<'_>) -> Result<(), Error> {
    tri! {
        err,
        unsafe { c_funcs::nvim_del_keymap(LUA_INTERNAL_CALL, map_mode, lhs, &mut err) }
    }
}

pub fn nvim_del_mark(name: ThinString<'_>) -> Result<(), Error> {
    tri! {
        err,
        unsafe {
            c_funcs::nvim_del_mark(name, &mut err)
        }
    }
}
