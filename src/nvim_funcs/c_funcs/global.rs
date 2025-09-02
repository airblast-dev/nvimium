use crate::nvim_types::{
    Arena, Array, Boolean, Buffer, Channel, Dict, Error, Integer, NameSpace, Object,
    borrowed::Borrowed,
    func_types::{echo::Echo, keymap_mode::KeyMapMode},
    opts::{
        context::ContextOpts, echo::EchoOpts, eval_statusline::EvalStatusLineOpts,
        get_hl::GetHlOpts, get_hl_ns::GetHlNsOpts, get_mark::GetMarkOpts, open_term::OpenTermOpts,
        paste::PastePhase, select_popupmenu_item::SelectPopupMenuOpts, set_hl::SetHlOpts,
        set_keymap::SetKeymapOpts,
    },
    returns::utils::ArrayOf,
    string::{OwnedThinString, ThinString},
    tab_page::TabPage,
    window::Window,
};
use core::mem::MaybeUninit;
use std::mem::ManuallyDrop;

// Some of the neovim functions do not accept a null pointer with strings and call functions
// such as strdup using the provided pointer. While this isn't a problem for strings constructed in
// nvimium, it is a problem if the neovim decides to return a null pointing string and the user
// provides it as an argument to a neovim function. [`AsThinString`] guarantees that the returned
// value never contains a null pointer this means passing it to an FFI boundary is always safe.
//
// TLDR; every function here can only accept a ThinString as its string type.
unsafe extern "C" {
    pub fn nvim_chan_send(chan: Channel, bytes: ThinString<'_>, err: *mut Error);
    pub fn nvim_create_buf(
        listed: Boolean,
        scratch: Boolean,
        err: *mut Error,
    ) -> MaybeUninit<Buffer>;
    pub fn nvim_del_current_line(arena: *mut Arena, err: *mut Error);
    pub fn nvim_del_keymap(
        chan: Channel,
        map_mode: KeyMapMode,
        lhs: ThinString<'_>,
        err: *mut Error,
    );
    pub fn nvim_del_mark(name: ThinString<'_>, err: *mut Error) -> MaybeUninit<Boolean>;
    pub fn nvim_del_var(var_name: ThinString<'_>, err: *mut Error);
    pub fn nvim_echo(
        chunks: Borrowed<'_, Echo>,
        history: bool,
        opts: *const EchoOpts,
        err: *mut Error,
    );
    #[deprecated]
    pub fn nvim_err_write(s: ThinString<'_>);
    #[deprecated]
    pub fn nvim_err_writeln(s: ThinString<'_>);
    pub fn nvim_eval_statusline(
        s: ThinString<'_>,
        opts: *const EvalStatusLineOpts<'_>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dict>;
    pub fn nvim_exec_lua(
        code: ThinString<'_>,
        args: Borrowed<'_, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_feedkeys(keys: ThinString<'_>, mode: ThinString<'_>, escape_ks: Boolean);
    /// Returns a shared value, caller must clone to mutate the value
    pub fn nvim_get_chan_info(
        channel_id: Channel,
        chan: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<ManuallyDrop<Dict>>;
    pub fn nvim_get_color_by_name(name: ThinString<'_>) -> Integer;
    // the color names returned are not owned, to avoid freeing a const value deal with the
    // deallocation of the Dict manually
    pub fn nvim_get_color_map(arena: *mut Arena) -> ManuallyDrop<Dict>;
    pub fn nvim_get_context(
        opts: *const ContextOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dict>;
    pub fn nvim_get_current_buf() -> Buffer;
    pub fn nvim_get_current_line(
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<OwnedThinString>;
    pub fn nvim_get_current_tabpage() -> TabPage;
    pub fn nvim_get_current_win() -> Window;
    // TODO: replace with custom struct or clone and partially free the returned values stored in
    // the dictionary have lifetimes that are known at runtime
    pub fn nvim_get_hl(
        ns_id: NameSpace,
        opts: *const GetHlOpts<'_>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dict>;
    pub fn nvim_get_hl_id_by_name(name: ThinString<'_>) -> Integer;
    pub fn nvim_get_hl_ns(opts: *const GetHlNsOpts, err: *mut Error) -> MaybeUninit<NameSpace>;
    pub fn nvim_get_keymap(mode: KeyMapMode, arena: *mut Arena) -> ManuallyDrop<Array>;
    pub fn nvim_get_mark(
        name: ThinString<'_>,
        opts: *const GetMarkOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_get_mode(arena: *mut Arena) -> ManuallyDrop<Dict>;
    pub fn nvim_get_proc(pid: Integer, arena: *mut Arena, err: *mut Error) -> MaybeUninit<Object>;
    pub fn nvim_get_proc_children(
        pid: Integer,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_get_runtime_file(
        name: ThinString<'_>,
        all: Boolean,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_get_var(
        name: ThinString<'_>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_get_vvar(
        name: ThinString<'_>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_input(channel: Channel, keys: ThinString<'_>) -> Integer;
    pub fn nvim_input_mouse(
        button: ThinString<'_>,
        action: ThinString<'_>,
        modifier: ThinString<'_>,
        grid: Integer,
        row: Integer,
        col: Integer,
        err: *mut Error,
    );
    pub fn nvim_list_bufs(arena: *mut Arena) -> ManuallyDrop<ArrayOf<Buffer>>;
    pub fn nvim_list_chans(arena: *mut Arena) -> ManuallyDrop<ArrayOf<Integer>>;
    pub fn nvim_list_runtime_paths(arena: *mut Arena, err: *mut Error) -> MaybeUninit<Array>;
    pub fn nvim_list_tabpages(arena: *mut Arena) -> ManuallyDrop<ArrayOf<TabPage>>;
    pub fn nvim_list_uis(arena: *mut Arena) -> ManuallyDrop<Array>;
    pub fn nvim_list_wins(arena: *mut Arena) -> ManuallyDrop<ArrayOf<Window>>;
    pub fn nvim_load_context(dict: Borrowed<'_, Dict>, err: *mut Error) -> MaybeUninit<Object>;
    pub fn nvim_open_term(
        buffer: Buffer,
        opts: *mut OpenTermOpts,
        err: *mut Error,
    ) -> MaybeUninit<Channel>;
    pub fn nvim_paste(
        channel: Channel,
        src: ThinString<'_>,
        crlf: Boolean,
        phase: PastePhase,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Boolean>;
    pub fn nvim_put(
        lines: Borrowed<'_, Array>,
        behavior: ThinString<'_>,
        after: Boolean,
        follow: Boolean,
        arena: *mut Arena,
        err: *mut Error,
    );
    pub fn nvim_replace_termcodes(
        s: ThinString<'_>,
        from_part: Boolean,
        do_lt: Boolean,
        special: Boolean,
    ) -> OwnedThinString;
    pub fn nvim_select_popupmenu_item(
        item: Integer,
        insert: Boolean,
        finish: Boolean,
        opts: *const SelectPopupMenuOpts,
        err: *mut Error,
    );
    pub fn nvim_set_current_buf(buf: Buffer, err: *mut Error);
    pub fn nvim_set_current_dir(dir: ThinString<'_>, err: *mut Error);
    pub fn nvim_set_current_line(line: ThinString<'_>, arena: *mut Arena, err: *mut Error);
    pub fn nvim_set_current_tabpage(tp: TabPage, err: *mut Error);
    pub fn nvim_set_current_win(win: Window, err: *mut Error);
    // unlike other options this theoretically might be mutated (url field)
    pub fn nvim_set_hl(
        chan: Channel,
        ns: NameSpace,
        name: ThinString<'_>,
        val: *mut SetHlOpts,
        err: *mut Error,
    );
    pub fn nvim_set_hl_ns(ns: NameSpace, err: *mut Error);
    pub fn nvim_set_hl_ns_fast(ns: NameSpace, err: *mut Error);
    pub fn nvim_set_keymap(
        chan: Channel,
        mode: KeyMapMode,
        lhs: ThinString<'_>,
        rhs: ThinString<'_>,
        opts: *mut SetKeymapOpts,
        err: *mut Error,
    );
    pub fn nvim_set_var(name: ThinString<'_>, obj: Borrowed<'_, Object>, err: *mut Error);
    pub fn nvim_set_vvar(name: ThinString<'_>, obj: Borrowed<'_, Object>, err: *mut Error);
    pub fn nvim_strwidth(name: ThinString<'_>, err: *mut Error) -> MaybeUninit<Integer>;
}
