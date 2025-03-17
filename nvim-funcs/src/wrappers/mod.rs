use std::{mem::ManuallyDrop, ops::DerefMut};

use macros::tri;
use nvim_types::{
    array::Array,
    borrowed::Borrowed,
    buffer::Buffer,
    call_site::Channel,
    dictionary::Dictionary,
    error::Error,
    func_types::{feedkeys::FeedKeysMode, keymap_mode::KeyMapMode},
    namespace::NameSpace,
    object::Object,
    opts::{
        echo::EchoOpts, eval_statusline::EvalStatusLineOpts, get_hl::GetHlOpts,
        get_hl_ns::GetHlNsOpts, get_mark::GetMarkOpts, open_term::OpenTermOpts, paste::PastePhase,
        select_popupmenu_item::SelectPopupMenuOpts, set_client_info::ClientKind, set_hl::SetHlOpts,
        set_keymap::SetKeymapOpts,
    },
    returns::{
        channel_info::ChannelInfo, color_map::ColorMap, eval_statusline::EvalStatusLineDict,
        get_mode::Mode,
    },
    string::{AsThinString, OwnedThinString},
    tab_page::TabPage,
    window::Window,
    Boolean, Integer,
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
        unsafe { c_funcs::nvim_create_buf(listed, scratch, &mut err) },
        Ok(buf) => Ok(unsafe{buf.assume_init()})
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
        unsafe { c_funcs::nvim_del_keymap(Channel::LUA_INTERNAL_CALL, map_mode, lhs.as_thinstr(), &mut err) }
    }
}

pub fn nvim_del_mark<S: AsThinString>(name: S) -> Result<Boolean, Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_del_mark(name.as_thinstr(), &mut err)
        },
        Ok(b) => Ok(unsafe{b.assume_init()})
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
    unsafe { c_funcs::nvim_get_api_info(Channel::LUA_INTERNAL_CALL, core::ptr::null_mut()) }
}

pub fn nvim_get_chan_info(channel_id: Channel, chan: Integer) -> Result<ChannelInfo, Error> {
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

pub fn nvim_get_hl<S: AsThinString>(ns: NameSpace, opts: &GetHlOpts) -> Result<Dictionary, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_hl(ns, opts, core::ptr::null_mut(), &mut err) },
        Ok(dict) => {
            // TODO: might be leaking some memory here
            let mut dict = ManuallyDrop::new(unsafe { dict.assume_init() });
            let res = Ok(ManuallyDrop::into_inner(dict.clone()));
            unsafe {
                dict.0.set_len(0);
                ManuallyDrop::drop(&mut dict);
            };

            res
        }
    }
}

pub fn nvim_get_hl_ns(opts: &GetHlNsOpts) -> Result<NameSpace, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_hl_ns(opts, &mut err) },
        Ok(ns) => Ok( unsafe { ns.assume_init() })
    }
}

pub fn nvim_get_keymap(mode: KeyMapMode) -> Array {
    let arr = unsafe { c_funcs::nvim_get_keymap(mode, core::ptr::null_mut()) };
    // TODO: fix memory leak, probably should create a specialized struct as well
    ManuallyDrop::into_inner(arr.clone())
}

pub fn nvim_get_mark<S: AsThinString>(name: S) -> Result<Array, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_mark(name.as_thinstr(), &GetMarkOpts::default(), core::ptr::null_mut(), &mut err)},
        Ok(arr) => {
            // TODO: fix memory leak, probably should create a specialized struct as well
            let arr = unsafe { ManuallyDrop::new(arr.assume_init()) };
            let ret = ManuallyDrop::into_inner(arr.clone());
            Ok(ret)
        }
    }
}

pub fn nvim_get_mode() -> Mode {
    let mut dict = unsafe { c_funcs::nvim_get_mode(core::ptr::null_mut()) };
    let mode = dict
        .remove_skip_key_drop("mode")
        .map(|m| {
            if let Object::String(s) = m {
                s
            } else {
                panic!("unexpected object type returned from nvim_get_mode for \"mode\" key");
            }
        })
        .expect("\"mode\" key missing in nvim_get_mode Dictionary");
    let blocking = dict
        .remove_skip_key_drop("blocking")
        .map(|b| {
            if let Object::Bool(b) = b {
                b
            } else {
                panic!("unexpected object type returned from nvim_get_mode for \"blocking\" key");
            }
        })
        .expect("\"blocking\" key missing in nvim_get_mode Dictionary");

    Mode { mode, blocking }
}

pub fn nvim_get_proc(pid: Integer) -> Result<Option<Dictionary>, Error> {
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_proc(pid, core::ptr::null_mut(), &mut err) },
        Ok(obj) => {
            let obj = unsafe { ManuallyDrop::new(obj.assume_init()) };
            let cpy = ManuallyDrop::into_inner(obj.clone());
            match cpy {
                Object::Null => Ok(None),
                Object::Dict(d) => Ok(Some(d)),
                _ => unreachable!("unknown object kind returned from nvim_get_proc")
            }
        }
    }
}

pub fn nvim_get_proc_children(pid: Integer) -> Result<Array, Error> {
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_proc_children(pid, core::ptr::null_mut(), &mut err) },
        Ok(obj) => {
            let obj = unsafe { ManuallyDrop::new(obj.assume_init()) };
            Ok(ManuallyDrop::into_inner(obj.clone()))
        }
    }
}

pub fn nvim_get_runtime_file<S: AsThinString>(name: S, all: Boolean) -> Result<Array, Error> {
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_runtime_file(name.as_thinstr(), all, core::ptr::null_mut(), &mut err) },
        Ok(arr) => {
            let arr = unsafe { ManuallyDrop::new(arr.assume_init()) };
            Ok(ManuallyDrop::into_inner(arr.clone()))
        }
    }
}

pub fn nvim_get_var<S: AsThinString>(name: S) -> Result<Object, Error> {
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_var(name.as_thinstr(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => {
            let obj = unsafe { ManuallyDrop::new(obj.assume_init()) };
            Ok(ManuallyDrop::into_inner(obj.clone()))
        }
    }
}

pub fn nvim_get_vvar<S: AsThinString>(name: S) -> Result<Object, Error> {
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_get_vvar(name.as_thinstr(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => {
            let obj = unsafe { ManuallyDrop::new(obj.assume_init()) };
            Ok(ManuallyDrop::into_inner(obj.clone()))
        }
    }
}

pub fn nvim_input<S: AsThinString>(keys: S) -> Integer {
    unsafe { c_funcs::nvim_input(Channel::LUA_INTERNAL_CALL, keys.as_thinstr()) }
}

pub fn nvim_input_mouse<S: AsThinString, S1: AsThinString, S2: AsThinString>(
    button: S,
    action: S1,
    modifier: S2,
    grid: Integer,
    row: Integer,
    col: Integer,
) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_input_mouse(
                button.as_thinstr(),
                action.as_thinstr(),
                modifier.as_thinstr(),
                grid,
                row,
                col,
                &mut err,
            )
        }
    }
}

pub fn nvim_list_bufs() -> Array {
    unsafe { c_funcs::nvim_list_bufs(core::ptr::null_mut()) }
}

pub fn nvim_list_chans() -> Array {
    unsafe { c_funcs::nvim_list_chans(core::ptr::null_mut()) }
}

pub fn nvim_list_runtime_paths() -> Result<Array, Error> {
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_list_runtime_paths(core::ptr::null_mut(), &mut err) },
        Ok(arr) => {
            let arr = unsafe { ManuallyDrop::new(arr.assume_init()) };
            Ok(ManuallyDrop::into_inner(arr.clone()))
        }
    }
}

pub fn nvim_list_tabpages() -> Array {
    unsafe { c_funcs::nvim_list_tabpages(core::ptr::null_mut()) }
}

pub fn nvim_list_uis() -> Array {
    // TODO: might be multiple memory leaks here
    unsafe {
        let uis = c_funcs::nvim_list_uis(core::ptr::null_mut());
        ManuallyDrop::into_inner(uis.clone())
    }
}

pub fn nvim_list_wins() -> Array {
    unsafe { c_funcs::nvim_list_wins(core::ptr::null_mut()) }
}

pub fn nvim_load_context(ctx: &Dictionary) -> Result<Object, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_load_context(ctx.into(), &mut err) },
        Ok(obj) => Ok(unsafe{obj.assume_init()})
    }
}

pub fn nvim_open_term(buf: Buffer, opts: &OpenTermOpts) -> Result<Integer, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_open_term(buf, opts, &mut err) },
        Ok(i) => Ok(unsafe{i.assume_init()})
    }
}

pub fn nvim_paste<S: AsThinString>(
    src: S,
    crlf: Boolean,
    phase: PastePhase,
) -> Result<Boolean, Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_paste(
                Channel::LUA_INTERNAL_CALL,
                src.as_thinstr(),
                crlf,
                phase,
                core::ptr::null_mut(),
                &mut err
            )
        },
        Ok(b) => Ok(unsafe{b.assume_init()})
    }
}

pub fn nvim_put<S: AsThinString>(
    arr: &Array,
    r#type: S,
    after: Boolean,
    follow: Boolean,
) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_put(arr.into(), r#type.as_thinstr(), after, follow, core::ptr::null_mut(), &mut err);
        }
    }
}

pub fn nvim_replace_termcodes<S: AsThinString>(
    s: S,
    from_part: Boolean,
    do_lt: Boolean,
    special: Boolean,
) -> OwnedThinString {
    unsafe { c_funcs::nvim_replace_termcodes(s.as_thinstr(), from_part, do_lt, special) }
}

pub fn nvim_select_popupmenu_item(
    item: Integer,
    insert: Boolean,
    finish: Boolean,
    opts: &SelectPopupMenuOpts,
) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_select_popupmenu_item(item, insert, finish, opts, &mut err) }
    }
}

pub fn nvim_set_client_info<S: AsThinString>(
    name: S,
    version: Borrowed<'_, Dictionary>,
    kind: ClientKind,
    methods: &Dictionary,
    attributes: &Dictionary,
) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_set_client_info(
                name.as_thinstr(),
                version,
                kind,
                methods.into(),
                attributes.into(),
                core::ptr::null_mut(),
                &mut err
            );
        }
    }
}

pub fn nvim_set_current_buf(buf: Buffer) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_current_buf(buf, &mut err); }
    }
}

pub fn nvim_set_current_dir<S: AsThinString>(dir: S) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_current_dir(dir.as_thinstr(), &mut err) }
    }
}

pub fn nvim_set_current_line<S: AsThinString>(line: S) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_current_line(line.as_thinstr(), core::ptr::null_mut(), &mut err) }
    }
}

pub fn nvim_set_current_tabpage(page: TabPage) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_current_tabpage(page, &mut err); }
    }
}

pub fn nvim_set_current_win(win: Window) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_current_win(win, &mut err); }
    }
}

pub fn nvim_set_hl<S: AsThinString>(ns: NameSpace, name: S, opts: &SetHlOpts) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe {
            c_funcs::nvim_set_hl(
                Channel::LUA_INTERNAL_CALL,
                ns, name.as_thinstr(),
                // may technically be mutated in neovim due to the url field
                (opts as *const SetHlOpts) as *mut SetHlOpts,
                &mut err
            );
        }
    }
}

pub fn nvim_set_hl_ns(ns: NameSpace) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_hl_ns(ns, &mut err) }
    }
}

pub fn nvim_set_hl_ns_fast(ns: NameSpace) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_hl_ns_fast(ns, &mut err) }
    }
}

pub fn nvim_set_keymap<S: AsThinString, S1: AsThinString>(
    mode: KeyMapMode,
    lhs: S,
    rhs: S1,
    opts: &SetKeymapOpts,
) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_keymap(Channel::LUA_INTERNAL_CALL, mode, lhs.as_thinstr(), rhs.as_thinstr(), opts, &mut err); }
    }
}

pub fn nvim_set_var<S: AsThinString>(s: S, obj: &Object) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_var(s.as_thinstr(), obj.into(), &mut err); }
    }
}

pub fn nvim_set_vvar<S: AsThinString>(s: S, obj: &Object) -> Result<(), Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_set_vvar(s.as_thinstr(), obj.into(), &mut err); }
    }
}

pub fn nvim_strwidth<S: AsThinString>(s: S) -> Result<Integer, Error> {
    tri! {
        let mut err;
        unsafe { c_funcs::nvim_strwidth(s.as_thinstr(), &mut err) },
        Ok(len) => Ok(unsafe{len.assume_init()})
    }
}

pub fn nvim_exec<S: AsThinString>(src: S, output: Boolean) -> Result<(), Error> {
    unsafe {
        tri! {
            let mut err;
            c_funcs::nvim_exec(Channel::LUA_INTERNAL_CALL, src.as_thinstr(), output, &mut err),
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

    #[nvim_test_macro::nvim_test(exit_call = nvim_exec)]
    pub fn test_nvim_strwidth() {
        let width = nvim_strwidth(c"".as_thinstr()).unwrap();
        assert_eq!(0, width);
        let width = nvim_strwidth(c"Hello".as_thinstr()).unwrap();
        assert_eq!(5, width);
    }
}
