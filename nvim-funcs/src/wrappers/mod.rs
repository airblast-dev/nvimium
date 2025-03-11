use std::{mem::ManuallyDrop, ops::DerefMut};

use macros::tri;
use nvim_types::{
    array::Array,
    borrowed::Borrowed,
    buffer::Buffer,
    call_site::LUA_INTERNAL_CALL,
    dictionary::Dictionary,
    error::Error,
    func_types::{feedkeys::FeedKeysMode, keymap_mode::KeyMapMode},
    object::Object,
    opts::{
        echo::EchoOpts, eval_statusline::EvalStatusLineOpts, get_hl::GetHlOpts,
        get_hl_ns::GetHlNsOpts, get_mark::GetMarkOpts,
    },
    returns::{
        channel_info::ChannelInfo, color_map::ColorMap, eval_statusline::EvalStatusLineDict,
    },
    string::{AsThinString, OwnedThinString},
    tab_page::TabPage,
    window::Window,
    Boolean, Integer, NameSpaceId,
};

// TODO: many of the functions exposed use static mutability internally
// calling many of these functions are unsound so add a check to ensure that functions that mutate
// static variables dont get called outside of neovim events

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
pub fn nvim_eval_statusline<S: AsThinString>(
    s: S,
    opts: &EvalStatusLineOpts,
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

pub fn nvim_feedkeys<S: AsThinString>(keys: S, mode: &FeedKeysMode, escape_ks: Boolean) {
    unsafe {
        c_funcs::nvim_feedkeys(keys.as_thinstr(), mode.as_thinstr(), escape_ks);
    }
}

pub fn nvim_get_api_info() -> Borrowed<'static, Array> {
    unsafe { c_funcs::nvim_get_api_info() }
}

pub fn nvim_get_chan_info(channel_id: u64, chan: Integer) -> Result<ChannelInfo, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_chan_info(channel_id, chan, core::ptr::null_mut(), &mut err) },
        Ok(ret) => Ok(unsafe { ChannelInfo::from_c_func_ret(&ret.assume_init()) })
    }
}

pub fn nvim_get_color_by_name<S: AsThinString>(name: S) -> Option<Integer> {
    let i = unsafe { c_funcs::nvim_get_color_by_name(name.as_thinstr()) };
    Some(i).filter(|i| *i != -1)
}

pub fn nvim_get_color_map() -> ColorMap {
    if !ColorMap::is_loaded() {
        #[cold]
        #[inline(never)]
        fn once() {}
        once();
        let mut color_dict = unsafe { c_funcs::nvim_get_color_map(core::ptr::null_mut()) };
        let cm = ColorMap::from_c_func_ret(color_dict.deref_mut());
        assert_eq!(color_dict.len(), 0);
        unsafe { ManuallyDrop::drop(&mut color_dict) };
        cm
    } else {
        ColorMap::initialized()
    }
}

pub fn nvim_get_current_buf() -> Buffer {
    unsafe { c_funcs::nvim_get_current_buf() }
}

pub fn nvim_get_current_line() -> Result<OwnedThinString, Error> {
    unsafe {
        tri! {
            let mut err;
            c_funcs::nvim_get_current_line(core::ptr::null_mut(), &mut err),
            Ok(res) => Ok(res.assume_init())
        }
    }
}

pub fn nvim_get_current_tabpage() -> TabPage {
    unsafe { c_funcs::nvim_get_current_tabpage() }
}

pub fn nvim_get_current_win() -> Window {
    unsafe { c_funcs::nvim_get_current_win() }
}

pub fn nvim_get_hl<S: AsThinString>(
    ns: NameSpaceId,
    opts: &GetHlOpts,
) -> Result<Dictionary, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_hl(ns, opts, core::ptr::null_mut(), &mut err) },
        Ok(dict) => {
            // TODO: might be leaking some memory here
            let mut dict = ManuallyDrop::new(unsafe { dict.assume_init() });
            let res = Ok(ManuallyDrop::into_inner(dict.clone()));
            unsafe {
                dict.kvec_mut().set_len(0);
                ManuallyDrop::drop(&mut dict);
            };

            res
        }
    }
}

pub fn nvim_get_hl_ns(opts: &GetHlNsOpts) -> Result<NameSpaceId, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_hl_ns(opts, &mut err) },
        Ok(ns) => Ok( unsafe { ns.assume_init() })
    }
}

pub fn nvim_get_keymap(mode: KeyMapMode) -> Array {
    let arr = unsafe { c_funcs::nvim_get_keymap(mode, core::ptr::null_mut()) };
    ManuallyDrop::into_inner(arr.clone())
}

pub fn nvim_get_mark<S: AsThinString>(name: S) -> Result<Array, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_mark(name.as_thinstr(), &GetMarkOpts::default(), core::ptr::null_mut(), &mut err)},
        Ok(arr) => {
            let arr = unsafe { ManuallyDrop::new(arr.assume_init()) };
            let ret = ManuallyDrop::into_inner(arr.clone());
            Ok(ret)
        }
    }
}

pub fn nvim_exec<S: AsThinString>(src: S, output: Boolean) -> Result<(), Error> {
    unsafe {
        tri! {
            let mut err;
            c_funcs::nvim_exec(LUA_INTERNAL_CALL, src.as_thinstr(), output, &mut err),
        }
    }
}

#[cfg(feature = "testing")]
mod tests {
    use super::*;
    use nvim_types::string::String;

    #[nvim_test_macro::nvim_test(exit_call = nvim_exec)]
    pub fn test_nvim_create_current_buf() {
        let buf = nvim_get_current_buf();
        assert_eq!(buf.as_int(), 1);
        let buf = nvim_create_buf(true, true).unwrap();
        assert_eq!(buf.as_int(), 2);
    }

    #[nvim_test_macro::nvim_test(exit_call = nvim_exec)]
    pub fn test_nvim_get_color_map() {
        let map = nvim_get_color_map();
        let color = map
            .get_with_name(String::from("Blue").as_thinstr())
            .expect("color not found");
        assert_eq!([0, 0, 255], color);
        let color = map
            .get_with_name(String::from("Red").as_thinstr())
            .expect("color not found");
        assert_eq!([255, 0, 0], color);
    }
}
