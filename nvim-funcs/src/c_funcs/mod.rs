use core::mem::MaybeUninit;
use nvim_types::{
    array::Array,
    borrowed::Borrowed,
    buffer::Buffer,
    dictionary::Dictionary,
    error::Error,
    func_types::keymap_mode::KeyMapMode,
    object::Object,
    opts::{
        echo::EchoOpts, eval_statusline::EvalStatusLineOpts, get_hl::GetHlOpts,
        get_hl_ns::GetHlNsOpts, get_mark::GetMarkOpts,
    },
    string::{OwnedThinString, ThinString},
    tab_page::TabPage,
    window::Window,
    Arena, Boolean, Integer, NameSpaceId,
};
use std::mem::ManuallyDrop;

// Some of the neovim functions do not accept a null pointer with strings and call functions
// such as strdup using the provided pointer. While this isn't a problem for strings constructed in
// nvimium, it is a problem if the neovim decides to return a null pointing string and the user
// provides it as an argument to a neovim function. [`AsThinString`] guarantees that the returned
// value never contains a null pointer this means passing it to an FFI boundary is always safe.
//
// TLDR; every function here can only accept a ThinString as its string type.
extern "C" {
    pub fn nvim_create_buf(listed: Boolean, scratch: Boolean, err: *mut Error) -> Buffer;
    pub fn nvim_del_current_line(arena: *mut Arena, err: *mut Error);
    pub fn nvim_del_keymap<'a>(
        chan: u64,
        map_mode: KeyMapMode,
        lhs: ThinString<'a>,
        err: *mut Error,
    );
    pub fn nvim_del_mark<'a>(name: ThinString<'a>, err: *mut Error) -> Boolean;
    pub fn nvim_del_var<'a>(var_name: ThinString<'a>, err: *mut Error);
    // TODO: Array<Array<[String; 2]>>
    pub fn nvim_echo<'a>(
        chunks: Borrowed<'a, Array>,
        history: bool,
        opts: *const EchoOpts,
        err: *mut Error,
    );
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
        args: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_feedkeys<'a>(keys: ThinString<'a>, mode: ThinString<'a>, escape_ks: Boolean);
    /// Returns a shared value, caller must clone to mutate the value
    pub fn nvim_get_api_info() -> Borrowed<'static, Array>;
    pub fn nvim_get_chan_info(
        channel_id: u64,
        chan: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<ManuallyDrop<Dictionary>>;
    pub fn nvim_get_color_by_name<'a>(name: ThinString<'a>) -> Integer;
    // the color names returned are not owned, to avoid freeing a const value deal with the
    // deallocation of the Dictionary manually
    pub fn nvim_get_color_map(arena: *mut Arena) -> ManuallyDrop<Dictionary>;
    pub fn nvim_get_current_buf() -> Buffer;
    pub fn nvim_get_current_line(
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<OwnedThinString>;
    pub fn nvim_get_current_tabpage() -> TabPage;
    pub fn nvim_get_current_win() -> Window;
    // TODO: replace with custom struct or clone and partially free the returned values stored in
    // the dictionary have lifetimes that are known at runtime
    pub fn nvim_get_hl<'a>(
        ns_id: NameSpaceId,
        opts: *const GetHlOpts<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dictionary>;
    pub fn nvim_get_hl_ns(opts: *const GetHlNsOpts, err: *mut Error) -> MaybeUninit<NameSpaceId>;
    pub fn nvim_get_keymap(mode: KeyMapMode, arena: *mut Arena) -> ManuallyDrop<Array>;
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
    pub fn nvim_load_context<'a>(
        dict: Borrowed<'a, Dictionary>,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    // TODO
    pub fn nvim_open_term(buffer: Buffer, opts: *const OpenTermOpts);
    pub fn nvim_exec<'a>(channel_id: u64, src: ThinString<'a>, output: Boolean, err: *mut Error);
}
