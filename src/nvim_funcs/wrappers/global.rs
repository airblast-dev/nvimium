use crate::{
    nvim_funcs::c_funcs::global::{self, nvim_chan_send},
    nvim_types::{
        Arena, ThinString,
        returns::{get_hl::HighlightGroups, get_keymap::Keymaps},
    },
};
use std::{mem::ManuallyDrop, ops::DerefMut};

use crate::nvim_types::{
    Array, AsThinString, Boolean, Buffer, Channel, Dict, Error, Integer, NameSpace, Object,
    OwnedThinString,
    func_types::{echo::Echo, feedkeys::FeedKeysMode, keymap_mode::KeyMapMode},
    opts::{
        context::ContextOpts, echo::EchoOpts, eval_statusline::EvalStatusLineOpts,
        get_hl::GetHlOpts, get_hl_ns::GetHlNsOpts, get_mark::GetMarkOpts, open_term::OpenTermOpts,
        paste::PastePhase, select_popupmenu_item::SelectPopupMenuOpts, set_hl::SetHlOpts,
        set_keymap::SetKeymapOpts,
    },
    returns::{
        channel_info::ChannelInfo, color_map::ColorMap, context::Context,
        eval_statusline::EvalStatusLine, get_mode::Mode,
    },
    tab_page::TabPage,
    window::Window,
};
use crate::tri;
use thread_lock::call_check;

pub fn chan_send<S: AsThinString>(chan: Channel, bytes: S) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { nvim_chan_send(chan, bytes.as_thinstr(), &mut err) },
    }
}

pub fn create_buf(listed: Boolean, scratch: Boolean) -> Result<Buffer, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_create_buf(listed, scratch, &mut err) },
        Ok(buf) => Ok(unsafe{buf.assume_init()})
    }
}

pub fn del_current_line() -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {global::nvim_del_current_line(core::ptr::null_mut(), &mut err) },
    }
}

pub fn del_keymap<S: AsThinString>(map_mode: KeyMapMode, lhs: S) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_del_keymap(Channel::LUA_INTERNAL_CALL, map_mode, lhs.as_thinstr(), &mut err) }
    }
}

pub fn del_mark<S: AsThinString>(name: S) -> Result<Boolean, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            global::nvim_del_mark(name.as_thinstr(), &mut err)
        },
        Ok(b) => Ok(unsafe{b.assume_init()})
    }
}

pub fn del_var<S: AsThinString>(var: S) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            global::nvim_del_var(var.as_thinstr(), &mut err);
        }
    }
}

pub fn echo<'a>(chunks: &'a Echo, history: Boolean, opts: &'a EchoOpts) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            global::nvim_echo(chunks.into(), history, opts, &mut err);
        }
    }
}

#[deprecated]
pub fn err_write<S: AsThinString>(s: S) {
    call_check();
    #[allow(deprecated)]
    unsafe {
        global::nvim_err_write(s.as_thinstr())
    };
}

#[deprecated]
pub fn err_writeln<S: AsThinString>(s: S) {
    call_check();
    #[allow(deprecated)]
    unsafe {
        global::nvim_err_writeln(s.as_thinstr())
    };
}

pub fn eval_statusline<S: AsThinString>(
    s: S,
    opts: &EvalStatusLineOpts,
) -> Result<EvalStatusLine, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_eval_statusline(s.as_thinstr(), opts, core::ptr::null_mut(), &mut err) },
        Ok(ret) => {
            let ret = unsafe { ManuallyDrop::new( ret.assume_init() ) };
            Ok(EvalStatusLine::from_c_func_ret(ret))
        }
    }
}

/// Execute a lua script
///
/// # Warning
///
/// This function is recommended to only be used in tests.
///
/// The returned [`Object`] has some limitations:
/// - If the [`Object`] contains a buffer, window or tabpage handle it will be returned as an
///   integer.
/// - The returned Object will not contain a LuaRef.
/// - Since we are passing a string as code, each call to this function will require the string to
///   be parsed and executed resulting in bad performance.
///
/// Due to these limitations you are encouraged to use the provided bindings or call functions from Lua
/// directly instead of using this function.
///
/// This function is mainly useful when writing tests.
pub fn exec_lua<S: AsThinString>(code: S, args: &Array) -> Result<Object, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_exec_lua(code.as_thinstr(), args.into(), core::ptr::null_mut(), &mut err) },
        Ok(ret) => {
            Ok(unsafe{ret.assume_init()})
        }
    }
}

pub fn feedkeys<S: AsThinString>(keys: S, mode: &FeedKeysMode, escape_ks: Boolean) {
    call_check();
    unsafe {
        global::nvim_feedkeys(keys.as_thinstr(), mode.as_thinstr(), escape_ks);
    }
}

pub fn get_chan_info(channel_id: Channel, chan: Integer) -> Result<ChannelInfo, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_get_chan_info(channel_id, chan, core::ptr::null_mut(), &mut err) },
        Ok(ret) => Ok(unsafe { ChannelInfo::from_c_func_ret(&ret.assume_init()) })
    }
}

pub fn get_color_by_name<S: AsThinString>(name: S) -> Option<Integer> {
    call_check();
    let i = unsafe { global::nvim_get_color_by_name(name.as_thinstr()) };
    Some(i).filter(|i| *i != -1)
}

/// Gets the full color map from Neovim
///
/// After the first call to this function the [`ColorMap`] is cached to avoid allocations on each
/// call.
pub fn get_color_map() -> ColorMap {
    call_check();
    if !ColorMap::is_loaded() {
        #[cold]
        #[inline(never)]
        fn once() {}
        once();
        let mut arena = Arena::EMPTY;
        let mut color_dict = unsafe { global::nvim_get_color_map(&mut arena) };
        let cm = ColorMap::from_c_func_ret(color_dict.deref_mut());
        assert_eq!(color_dict.len(), 0);
        cm
    } else {
        ColorMap::initialized()
    }
}

pub fn get_context(opts: &ContextOpts) -> Result<Context, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_get_context(opts, core::ptr::null_mut(), &mut err) },
        Ok(d) => Ok(unsafe {
            let mut d = ManuallyDrop::new(d.assume_init());
            let ctx = Context::from_c_func_ret(&mut d);
            ManuallyDrop::into_inner(d);
            ctx
        })
    }
}

pub fn get_current_buf() -> Buffer {
    call_check();
    unsafe { global::nvim_get_current_buf() }
}

pub fn get_current_line() -> Result<OwnedThinString, Error> {
    call_check();
    unsafe {
        tri! {
            let mut err;
            global::nvim_get_current_line(core::ptr::null_mut(), &mut err),
            Ok(res) => Ok(res.assume_init())
        }
    }
}

pub fn get_current_tabpage() -> TabPage {
    call_check();
    unsafe { global::nvim_get_current_tabpage() }
}

pub fn get_current_win() -> Window {
    call_check();
    unsafe { global::nvim_get_current_win() }
}

pub fn get_hl(ns: NameSpace, opts: &GetHlOpts) -> Result<HighlightGroups, Error> {
    call_check();
    let mut arena = Arena::EMPTY;
    tri! {
        let mut err;
        unsafe { global::nvim_get_hl(ns, opts, &mut arena, &mut err) },
        Ok(dict) => {
            let dict = ManuallyDrop::new(unsafe { dict.assume_init_ref() });
            let res = HighlightGroups::from_c_func_ret(&dict);


            Ok(res)
        }
    }
}

pub fn get_hl_id_by_name<S: AsThinString>(name: S) -> Integer {
    call_check();
    unsafe { global::nvim_get_hl_id_by_name(name.as_thinstr()) }
}

pub fn get_hl_ns(opts: &GetHlNsOpts) -> Result<NameSpace, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_get_hl_ns(opts, &mut err) },
        Ok(ns) => Ok( unsafe { ns.assume_init() })
    }
}

pub fn get_keymap(mode: KeyMapMode) -> Keymaps {
    call_check();
    let mut arena = Arena::EMPTY;
    let mut arr = unsafe { global::nvim_get_keymap(mode, &mut arena) };
    Keymaps::from_c_func_ret(&mut arr)
}

pub fn get_mark<S: AsThinString>(name: S) -> Result<Array, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_get_mark(name.as_thinstr(), &GetMarkOpts::default(), core::ptr::null_mut(), &mut err)},
        Ok(arr) => {
            // TODO: fix memory leak, probably should create a specialized struct as well
            let arr = unsafe { ManuallyDrop::new(arr.assume_init()) };
            let ret = ManuallyDrop::into_inner(arr.clone());
            Ok(ret)
        }
    }
}

pub fn get_mode() -> Mode {
    call_check();
    let mut dict = unsafe { global::nvim_get_mode(core::ptr::null_mut()) };
    Mode::from_c_func_ret(&mut dict)
}

pub fn get_proc(pid: Integer) -> Result<Option<Dict>, Error> {
    call_check();
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { global::nvim_get_proc(pid, core::ptr::null_mut(), &mut err) },
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

pub fn get_proc_children(pid: Integer) -> Result<Array, Error> {
    call_check();
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { global::nvim_get_proc_children(pid, core::ptr::null_mut(), &mut err) },
        Ok(obj) => {
            let obj = unsafe { ManuallyDrop::new(obj.assume_init()) };
            Ok(ManuallyDrop::into_inner(obj.clone()))
        }
    }
}

pub fn get_runtime_file<S: AsThinString>(name: S, all: Boolean) -> Result<Array, Error> {
    call_check();
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { global::nvim_get_runtime_file(name.as_thinstr(), all, core::ptr::null_mut(), &mut err) },
        Ok(arr) => {
            let arr = unsafe { ManuallyDrop::new(arr.assume_init()) };
            Ok(ManuallyDrop::into_inner(arr.clone()))
        }
    }
}

pub fn get_var<S: AsThinString>(name: S) -> Result<Object, Error> {
    call_check();
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { global::nvim_get_var(name.as_thinstr(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => {
            let obj = unsafe { ManuallyDrop::new(obj.assume_init()) };
            Ok(ManuallyDrop::into_inner(obj.clone()))
        }
    }
}

pub fn get_vvar<S: AsThinString>(name: S) -> Result<Object, Error> {
    call_check();
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { global::nvim_get_vvar(name.as_thinstr(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => {
            let obj = unsafe { ManuallyDrop::new(obj.assume_init()) };
            Ok(ManuallyDrop::into_inner(obj.clone()))
        }
    }
}

pub fn input<S: AsThinString>(keys: S) -> Integer {
    call_check();
    unsafe { global::nvim_input(Channel::LUA_INTERNAL_CALL, keys.as_thinstr()) }
}

pub fn input_mouse<S: AsThinString, S1: AsThinString, S2: AsThinString>(
    button: S,
    action: S1,
    modifier: S2,
    grid: Integer,
    row: Integer,
    col: Integer,
) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            global::nvim_input_mouse(
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

pub fn list_bufs() -> Array {
    call_check();
    unsafe { global::nvim_list_bufs(core::ptr::null_mut()) }
}

pub fn list_chans() -> Array {
    call_check();
    unsafe { global::nvim_list_chans(core::ptr::null_mut()) }
}

pub fn list_runtime_paths() -> Result<Array, Error> {
    call_check();
    // TODO: might be multiple memory leaks here
    tri! {
        let mut err;
        unsafe { global::nvim_list_runtime_paths(core::ptr::null_mut(), &mut err) },
        Ok(arr) => {
            let arr = unsafe { ManuallyDrop::new(arr.assume_init()) };
            Ok(ManuallyDrop::into_inner(arr.clone()))
        }
    }
}

pub fn list_tabpages() -> Array {
    call_check();
    unsafe { global::nvim_list_tabpages(core::ptr::null_mut()) }
}

pub fn list_uis() -> Array {
    call_check();
    // TODO: might be multiple memory leaks here
    unsafe {
        let uis = global::nvim_list_uis(core::ptr::null_mut());
        ManuallyDrop::into_inner(uis.clone())
    }
}

pub fn list_wins() -> Array {
    call_check();
    unsafe { global::nvim_list_wins(core::ptr::null_mut()) }
}

pub fn load_context(ctx: &Dict) -> Result<Object, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_load_context(ctx.into(), &mut err) },
        Ok(obj) => Ok(unsafe{obj.assume_init()})
    }
}

pub fn open_term(buf: Buffer, opts: &mut OpenTermOpts) -> Result<Integer, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_open_term(buf, opts, &mut err) },
        Ok(i) => Ok(unsafe{i.assume_init()})
    }
}

pub fn paste<S: AsThinString>(src: S, crlf: Boolean, phase: PastePhase) -> Result<Boolean, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            global::nvim_paste(
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

pub fn put<S: AsThinString>(
    arr: &Array,
    r#type: S,
    after: Boolean,
    follow: Boolean,
) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            global::nvim_put(arr.into(), r#type.as_thinstr(), after, follow, core::ptr::null_mut(), &mut err);
        }
    }
}

pub fn replace_termcodes<S: AsThinString>(
    s: S,
    from_part: Boolean,
    do_lt: Boolean,
    special: Boolean,
) -> OwnedThinString {
    call_check();
    unsafe { global::nvim_replace_termcodes(s.as_thinstr(), from_part, do_lt, special) }
}

pub fn select_popupmenu_item(
    item: Integer,
    insert: Boolean,
    finish: Boolean,
    opts: &SelectPopupMenuOpts,
) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_select_popupmenu_item(item, insert, finish, opts, &mut err) }
    }
}

pub fn set_current_buf(buf: Buffer) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_current_buf(buf, &mut err); }
    }
}

pub fn set_current_dir<S: AsThinString>(dir: S) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_current_dir(dir.as_thinstr(), &mut err) }
    }
}

pub fn set_current_line<S: AsThinString>(line: S) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_current_line(line.as_thinstr(), core::ptr::null_mut(), &mut err) }
    }
}

pub fn set_current_tabpage(page: TabPage) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_current_tabpage(page, &mut err); }
    }
}

pub fn set_current_win(win: Window) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_current_win(win, &mut err); }
    }
}

pub fn set_hl<S: AsThinString>(ns: NameSpace, name: S, opts: &SetHlOpts) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            global::nvim_set_hl(
                Channel::LUA_INTERNAL_CALL,
                ns, name.as_thinstr(),
                // may technically be mutated in neovim due to the url field
                (opts as *const SetHlOpts) as *mut SetHlOpts,
                &mut err
            );
        }
    }
}

pub fn set_hl_ns(ns: NameSpace) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_hl_ns(ns, &mut err) }
    }
}

pub fn set_hl_ns_fast(ns: NameSpace) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_hl_ns_fast(ns, &mut err) }
    }
}

pub fn set_keymap<S: AsThinString, S1: AsThinString>(
    mode: KeyMapMode,
    lhs: S,
    rhs: S1,
    opts: &mut SetKeymapOpts,
) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_keymap(Channel::LUA_INTERNAL_CALL, mode, lhs.as_thinstr(), rhs.as_thinstr(), opts, &mut err); }
    }
}

pub fn set_var<S: AsThinString>(s: S, obj: &Object) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_var(s.as_thinstr(), obj.into(), &mut err); }
    }
}

pub fn set_vvar<S: AsThinString>(s: S, obj: &Object) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_set_vvar(s.as_thinstr(), obj.into(), &mut err); }
    }
}

pub fn strwidth<S: AsThinString>(s: S) -> Result<Integer, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { global::nvim_strwidth(s.as_thinstr(), &mut err) },
        Ok(len) => Ok(unsafe{len.assume_init()})
    }
}

#[cfg(all(not(miri), feature = "testing"))]
mod tests {
    use crate as nvimium;
    use crate::nvim_funcs::vimscript::exec2;
    use crate::nvim_types::returns::get_keymap::Keymap;
    use crate::nvim_types::{
        Array, AsThinString, Dict, NvString, Object, OwnedThinString, Window,
        func_types::{
            echo::Echo,
            feedkeys::{FeedKeysMode, FeedKeysModeKind},
            keymap_mode::KeyMapMode,
        },
        kvec::KVec,
        opts::{
            context::ContextOpts, echo::EchoOpts, eval_statusline::EvalStatusLineOpts,
            exec::ExecOpts, set_keymap::SetKeymapOpts,
        },
    };
    use libc::{c_char, strstr};

    // calling `thread_lock::unlock` is safe as every test is spawned as a process with independent
    // threads
    //
    // only exclusion is when multithreading may being tested

    #[nvim_test::nvim_test]
    pub fn nvim_create_current_buf() {
        let buf = super::get_current_buf();
        assert_eq!(buf.as_int(), 1);
        let buf = super::create_buf(true, true).unwrap();
        assert_eq!(buf.as_int(), 2);
    }

    #[nvim_test::nvim_test]
    pub fn nvim_get_set_current_tabpage() {
        let tp = super::get_current_tabpage();
        assert_eq!(tp.as_int(), 1);
        super::set_current_tabpage(tp).unwrap();
        // TODO: cant test as feedkeys is lazy
    }

    #[nvim_test::nvim_test]
    pub fn nvim_get_set_current_window() {
        let w = super::get_current_win();
        assert_eq!(w.as_int(), 1000);
        super::set_current_win(w).unwrap();
        // TODO: cant test as feedkeys is lazy
    }

    #[nvim_test::nvim_test]
    pub fn nvim_set_get_del_current_line() {
        super::set_current_line(c"Hello World!").unwrap();
        let l = super::get_current_line().unwrap();
        assert_eq!(l, "Hello World!");
        super::del_current_line().unwrap();
        let l = super::get_current_line().unwrap();
        assert_eq!(l, "");
    }

    #[nvim_test::nvim_test]
    pub fn nvim_get_color_map() {
        let map = super::get_color_map();
        let color = map
            .get_with_name(NvString::from("Blue").as_thinstr())
            .expect("color not found");
        assert_eq!([0, 0, 255], color);
        let color = map
            .get_with_name(NvString::from("Red").as_thinstr())
            .expect("color not found");
        assert_eq!([255, 0, 0], color);
    }

    #[nvim_test::nvim_test]
    pub fn nvim_get_context() {
        let ctx = super::get_context(ContextOpts::default().list(Array(KVec::from_iter(
            [OwnedThinString::from("gvars")].map(Object::String),
        ))))
        .unwrap();

        // I have no idea how to actually test this properly as the exact gvars can change
        // order.
        //
        // As part of the test system NVIMIUM_PANIC_LOG_FILE global variable is set, so it has to be
        // stored somewhere in the gvars value
        let gvars = ctx.gvars.0;
        let found = gvars
            .into_iter()
            .filter_map(Object::into_string)
            .any(|s| unsafe {
                !core::ptr::eq(
                    strstr(
                        s.as_thinstr().as_ptr() as *const c_char,
                        c"NVIMIUM_PANIC_LOG_FILE".as_ptr() as *const c_char,
                    ),
                    core::ptr::null(),
                )
            });

        assert!(found);
    }

    #[nvim_test::nvim_test]
    pub fn nvim_set_get_delete_keymap() {
        // this test is kind of hacky
        //
        // any mapping result is only evaluated after yielding execution so instead we attempt to
        // create a mapping then delete it. if deleting the keymap fails it means the keymap wasn't
        // set/doesn't exist
        let keymaps = super::get_keymap(KeyMapMode::MODE_OP_PENDING);

        // we dont actually have to do this but this ensures that there isn't an already existing
        // keymap removing the risk of false positives
        let found = keymaps.maps.into_iter().any(|keymap| {
            Keymap {
                lhs: OwnedThinString::from("aasdsadasdasdasdasdas"),
                rhs: OwnedThinString::from(":lua vim.api.nvim_set_current_line('HELLOO')").into(),
                nowait: true,
                ..Default::default()
            } == keymap
        });
        assert!(!found);
        super::set_keymap(
            KeyMapMode::MODE_OP_PENDING,
            c"aasdsadasdasdasdasdas",
            c":lua vim.api.nvim_set_current_line('HELLOO')",
            SetKeymapOpts::default().noawait(true),
        )
        .unwrap();
        let keymaps = super::get_keymap(KeyMapMode::MODE_OP_PENDING);
        let found = keymaps.maps.into_iter().any(|keymap| {
            Keymap {
                lhs: OwnedThinString::from("aasdsadasdasdasdasdas"),
                rhs: OwnedThinString::from(":lua vim.api.nvim_set_current_line('HELLOO')").into(),
                nowait: true,
                ..Default::default()
            } == Keymap {
                lhs: keymap.lhs,
                rhs: keymap.rhs,
                nowait: keymap.nowait,
                ..Default::default()
            }
        });
        assert!(found);
        let res = super::del_keymap(KeyMapMode::MODE_OP_PENDING, c"aasdsadasdasdasdasdas");
        assert!(res.is_ok());

        // intentionally test a bad value as this should fail if the delete above succeedes
        let err =
            super::del_keymap(KeyMapMode::MODE_OP_PENDING, c"aasdsadasdasdasdasdas").unwrap_err();
        assert!(format!("{:?}", err) == r##"Exception: "E31: No such mapping""##)
    }

    #[nvim_test::nvim_test]
    pub fn nvim_set_get_del_var() {
        let var = Object::Dict(Dict::from_iter([
            ("apples", Object::Integer(22)),
            ("oranges", Object::Bool(true)),
        ]));
        super::set_var(c"apple_count", &var).unwrap();
        let ret_var = super::get_var(c"apple_count").unwrap();

        let expected = Dict::from_iter([
            ("oranges", Object::Bool(true)),
            ("apples", Object::Integer(22)),
        ]);

        assert_eq!(ret_var, Object::Dict(expected));

        super::del_var(c"apple_count").unwrap();

        let ret_var = super::get_var(c"apple_count").unwrap_err();
        assert_eq!(
            format!("{ret_var:?}"),
            r##"Validation: "Key not found: apple_count""##
        );

        let ret_var = super::del_var(c"apple_count").unwrap_err();
        assert_eq!(
            format!("{ret_var:?}"),
            r##"Validation: "Key not found: apple_count""##
        );
    }

    #[nvim_test::nvim_test]
    pub fn nvim_echo() {
        super::echo(&Echo::message(c"Hello!"), true, &EchoOpts::default()).unwrap();
        let mut opts = ExecOpts::default();
        let output = exec2(c":messages", opts.output(true)).unwrap();
        assert_eq!(
            output.get(c"output".as_thinstr()).unwrap(),
            &Object::String(OwnedThinString::from("Hello!"))
        );
    }

    #[nvim_test::nvim_test]
    pub fn nvim_eval_statusline() {
        let res =
            super::eval_statusline(c"Hello", EvalStatusLineOpts::default().fillchar(c"a")).unwrap();

        assert_eq!(res.chars, "Hello");
        assert_eq!(res.width, 5);
        assert!(res.highlights.is_none());

        // see the TODO above
        //
        // since some of the options are ignored for some reason, test what we can test for
        let err = super::eval_statusline(
            c"123",
            EvalStatusLineOpts::default().winid(Window::new(999)),
        )
        .unwrap_err();
        assert_eq!("Exception: \"unknown winid 999\"", err.to_string());
    }

    #[nvim_test::nvim_test]
    pub fn nvim_exec_lua() {
        let res = super::exec_lua(
            c"vim.api.nvim_create_buf(true, true)\nreturn 12",
            &Array::default(),
        )
        .unwrap();
        assert_eq!(res, Object::Integer(12));
        assert_eq!(super::list_bufs().len(), 2);
    }

    #[nvim_test::nvim_test(no_exit)]
    pub fn nvim_feedkeys() {
        // this test is kind of a hack
        //
        // nvim_feedkeys doesnt actually write to the buffer, but rather flushes it in the next
        // tick
        // this means we cannot read the buffer by calling nvim_get_current_line or any other
        // method
        // instead we set a keymap that matches the keys we have fed, once the string is "typed"
        // into the buffer the keymap triggers and exits neovim
        super::feedkeys(
            c"i123",
            &FeedKeysMode::from([FeedKeysModeKind::Typed]),
            false,
        );
        super::set_keymap(
            KeyMapMode::INSERT,
            c"3",
            c"<Esc>:qall!<CR>",
            &mut SetKeymapOpts::default(),
        )
        .unwrap();
        //super::nvim_exec_lua(
        //    c"vim.keymap.set('i', '3', function() vim.cmd([[qall!]]) end)",
        //    &Array::default(),
        //)
        //.unwrap();
    }

    #[nvim_test::nvim_test]
    pub fn nvim_get_color_by_name() {
        let color = super::get_color_by_name(c"Blue").unwrap();
        assert_eq!(color, 255);
    }

    #[nvim_test::nvim_test]
    pub fn nvim_strwidth() {
        let width = super::strwidth(c"".as_thinstr()).unwrap();
        assert_eq!(0, width);
        let width = super::strwidth(c"Hello".as_thinstr()).unwrap();
        assert_eq!(5, width);
    }
}
