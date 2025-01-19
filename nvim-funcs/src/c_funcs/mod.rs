use core::mem::MaybeUninit;
use std::mem::ManuallyDrop;

use nvim_types::{
    array::Array,
    buffer::Buffer,
    dictionary::Dictionary,
    error::Error,
    func_types::KeyMapMode,
    object::Object,
    opts::{
        echo::EchoOpts, eval_statusline::EvalStatusLineOpts, get_hl::GetHlOpts,
        get_hl_ns::GetHlNsOpts, get_mark::GetMarkOpts,
    },
    string::{OwnedThinString, ThinString},
    tab_page::TabPage,
    window::Window,
    Arena, Boolean, Integer,
};

// Any of the functions can only take a [`ThinString`] or [`OwnedThinString`]. As the layout and
// size of [`String`] is not the same.
extern "C" {
    pub fn nvim_create_buf(listed: Boolean, scratch: Boolean) -> Buffer;
    pub fn nvim_del_current_line(arena: *mut Arena, err: *mut Error);
    pub fn nvim_del_keymap<'a>(
        chan: u64,
        map_mode: KeyMapMode,
        lhs: ThinString<'a>,
        err: *mut Error,
    );
    pub fn nvim_del_mark<'a>(name: ThinString<'a>, err: *mut Error);
    pub fn nvim_del_var<'a>(var_name: ThinString<'a>, err: *mut Error);
    // Array<Array<[String; 2]>>
    pub fn nvim_echo<'a>(chunks: ManuallyDrop<Array>, history: bool, opts: *const EchoOpts);
    pub fn nvim_err_write<'a>(s: ThinString<'a>);
    pub fn nvim_err_writeln<'a>(s: ThinString<'a>);
    pub fn nvim_eval_statusline<'a>(
        s: ThinString<'a>,
        opts: *const EvalStatusLineOpts<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dictionary>;
    pub fn nvim_exec_lua<'a>(
        code: ThinString<'a>,
        args: Array,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    // TODO: replace mode type with its own struct
    pub fn nvim_feedkeys<'a>(keys: ThinString<'a>, mode: ThinString<'a>, escape_ks: Boolean);
    pub fn nvim_get_api_info() -> Array;
    pub fn nvim_get_chan_info(
        channel_id: u64,
        chan: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dictionary>;
    pub fn nvim_get_color_by_name<'a>(name: ThinString<'a>) -> Integer;
    pub fn nvim_get_color_map(arena: *mut Arena) -> MaybeUninit<Dictionary>;
    pub fn nvim_get_current_buf() -> Buffer;
    pub fn nvim_get_current_line(
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<OwnedThinString>;
    pub fn nvim_get_current_tabpage() -> TabPage;
    pub fn nvim_get_current_win() -> Window;
    pub fn nvim_get_hl<'a>(
        ns_id: Integer,
        opts: GetHlOpts<'a>,
        array: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dictionary>;
    pub fn nvim_get_hl_ns<'a>(opts: GetHlNsOpts, err: *mut Error) -> Integer;
    pub fn nvim_get_keymap(mode: KeyMapMode) -> MaybeUninit<Array>;
    pub fn nvim_get_mark<'a>(
        name: ThinString<'a>,
        opts: GetMarkOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_get_mode(arena: *mut Arena) -> Dictionary;
    pub fn nvim_get_proc(pid: Integer, arena: *mut Arena, err: *mut Error) -> MaybeUninit<Object>;
    pub fn nvim_get_proc_children(
        pid: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_get_runtime_file<'a>(
        name: ThinString<'a>,
        all: Boolean,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_get_var<'a>(
        name: ThinString<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_get_vvar<'a>(
        name: ThinString<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_input<'a>(keys: ThinString<'a>) -> Integer;
    pub fn nvim_input_mouse<'a>(
        button: ThinString<'a>,
        action: ThinString<'a>,
        modifier: ThinString<'a>,
        grid: Integer,
        row: Integer,
        col: Integer,
        err: *mut Error,
    );
    pub fn nvim_list_bufs(arena: *mut Arena) -> MaybeUninit<Array>;
    pub fn nvim_list_chans(arena: *mut Arena) -> MaybeUninit<Array>;
    pub fn nvim_list_runtime_paths(arena: *mut Arena, err: *mut Error) -> MaybeUninit<Array>;
    pub fn nvim_list_tabpages(arena: *mut Arena) -> MaybeUninit<Array>;
    pub fn nvim_list_uis(arena: *mut Arena) -> MaybeUninit<Array>;
    pub fn nvim_list_wins(arena: *mut Arena) -> MaybeUninit<Array>;
    pub fn nvim_load_context(dict: Dictionary, err: *mut Error) -> MaybeUninit<Object>;
    // TODO
    pub fn nvim_open_term(buffer: Buffer);
}
