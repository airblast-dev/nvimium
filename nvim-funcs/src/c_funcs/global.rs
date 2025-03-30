use core::mem::MaybeUninit;
use nvim_types::{
    array::Array,
    borrowed::Borrowed,
    buffer::Buffer,
    call_site::Channel,
    dictionary::Dictionary,
    error::Error,
    func_types::{echo::Echo, keymap_mode::KeyMapMode},
    namespace::NameSpace,
    object::Object,
    opts::{
        echo::EchoOpts, eval_statusline::EvalStatusLineOpts, get_hl::GetHlOpts,
        get_hl_ns::GetHlNsOpts, get_mark::GetMarkOpts, open_term::OpenTermOpts, paste::PastePhase,
        select_popupmenu_item::SelectPopupMenuOpts, set_client_info::ClientKind, set_hl::SetHlOpts,
        set_keymap::SetKeymapOpts,
    },
    string::{OwnedThinString, ThinString},
    tab_page::TabPage,
    window::Window,
    Arena, Boolean, Integer,
};
use std::mem::ManuallyDrop;

// Some of the neovim functions do not accept a null pointer with strings and call functions
// such as strdup using the provided pointer. While this isn't a problem for strings constructed in
// nvimium, it is a problem if the neovim decides to return a null pointing string and the user
// provides it as an argument to a neovim function. [`AsThinString`] guarantees that the returned
// value never contains a null pointer this means passing it to an FFI boundary is always safe.
//
// TLDR; every function here can only accept a ThinString as its string type.
unsafe extern "C" {
    pub fn nvim_create_buf(
        listed: Boolean,
        scratch: Boolean,
        err: *mut Error,
    ) -> MaybeUninit<Buffer>;
    pub fn nvim_del_current_line(arena: *mut Arena, err: *mut Error);
    pub fn nvim_del_keymap<'a>(
        chan: Channel,
        map_mode: KeyMapMode,
        lhs: ThinString<'a>,
        err: *mut Error,
    );
    pub fn nvim_del_mark<'a>(name: ThinString<'a>, err: *mut Error) -> MaybeUninit<Boolean>;
    pub fn nvim_del_var<'a>(var_name: ThinString<'a>, err: *mut Error);
    // TODO: Array<Array<[String; 2]>>
    pub fn nvim_echo<'a>(
        chunks: Borrowed<'a, Echo>,
        history: bool,
        opts: *const EchoOpts,
        err: *mut Error,
    );
    #[deprecated]
    pub fn nvim_err_write<'a>(s: ThinString<'a>);
    #[deprecated]
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
    pub fn nvim_get_api_info(channel_id: Channel, arena: *mut Arena) -> Borrowed<'static, Array>;
    pub fn nvim_get_chan_info(
        channel_id: Channel,
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
        ns_id: NameSpace,
        opts: *const GetHlOpts<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dictionary>;
    pub fn nvim_get_hl_ns(opts: *const GetHlNsOpts, err: *mut Error) -> MaybeUninit<NameSpace>;
    pub fn nvim_get_keymap(mode: KeyMapMode, arena: *mut Arena) -> ManuallyDrop<Array>;
    pub fn nvim_get_mark<'a>(
        name: ThinString<'a>,
        opts: *const GetMarkOpts,
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
    pub fn nvim_input<'a>(channel: Channel, keys: ThinString<'a>) -> Integer;
    pub fn nvim_input_mouse<'a>(
        button: ThinString<'a>,
        action: ThinString<'a>,
        modifier: ThinString<'a>,
        grid: Integer,
        row: Integer,
        col: Integer,
        err: *mut Error,
    );
    pub fn nvim_list_bufs(arena: *mut Arena) -> Array;
    pub fn nvim_list_chans(arena: *mut Arena) -> Array;
    pub fn nvim_list_runtime_paths(arena: *mut Arena, err: *mut Error) -> MaybeUninit<Array>;
    pub fn nvim_list_tabpages(arena: *mut Arena) -> Array;
    pub fn nvim_list_uis(arena: *mut Arena) -> ManuallyDrop<Array>;
    pub fn nvim_list_wins(arena: *mut Arena) -> Array;
    pub fn nvim_load_context<'a>(
        dict: Borrowed<'a, Dictionary>,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    // TODO
    pub fn nvim_open_term(
        buffer: Buffer,
        opts: *const OpenTermOpts,
        err: *mut Error,
    ) -> MaybeUninit<Integer>;
    pub fn nvim_paste<'a>(
        channel: Channel,
        src: ThinString<'a>,
        crlf: Boolean,
        phase: PastePhase,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Boolean>;
    pub fn nvim_put<'a>(
        lines: Borrowed<'a, Array>,
        behavior: ThinString<'a>,
        after: Boolean,
        follow: Boolean,
        arena: *mut Arena,
        err: *mut Error,
    );
    pub fn nvim_replace_termcodes<'a>(
        s: ThinString<'a>,
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
    pub fn nvim_set_client_info<'a>(
        name: ThinString<'a>,
        version: Borrowed<'a, Dictionary>,
        kind: ClientKind,
        methods: Borrowed<'a, Dictionary>,
        attributes: Borrowed<'a, Dictionary>,
        arena: *mut Arena,
        err: *mut Error,
    );
    pub fn nvim_set_current_buf(buf: Buffer, err: *mut Error);
    pub fn nvim_set_current_dir<'a>(dir: ThinString<'a>, err: *mut Error);
    pub fn nvim_set_current_line<'a>(line: ThinString<'a>, arena: *mut Arena, err: *mut Error);
    pub fn nvim_set_current_tabpage(tp: TabPage, err: *mut Error);
    pub fn nvim_set_current_win(win: Window, err: *mut Error);
    // unlike other options this theoretically might be mutated (url field)
    pub fn nvim_set_hl<'a>(
        chan: Channel,
        ns: NameSpace,
        name: ThinString<'a>,
        val: *mut SetHlOpts,
        err: *mut Error,
    );
    pub fn nvim_set_hl_ns(ns: NameSpace, err: *mut Error);
    pub fn nvim_set_hl_ns_fast(ns: NameSpace, err: *mut Error);
    pub fn nvim_set_keymap<'a>(
        chan: Channel,
        mode: KeyMapMode,
        lhs: ThinString<'a>,
        rhs: ThinString<'a>,
        opts: *const SetKeymapOpts,
        err: *mut Error,
    );
    pub fn nvim_set_var<'a>(name: ThinString<'a>, obj: Borrowed<'a, Object>, err: *mut Error);
    pub fn nvim_set_vvar<'a>(name: ThinString<'a>, obj: Borrowed<'a, Object>, err: *mut Error);
    pub fn nvim_strwidth<'a>(name: ThinString<'a>, err: *mut Error) -> MaybeUninit<Integer>;

    // these should come later
    // TODO: use proper opts type
    pub fn nvim_exec2<'a>(
        channel_id: Channel,
        src: ThinString<'a>,
        opts: Dictionary,
        err: *mut Error,
    );
    // TODO: use proper opts type
    pub fn nvim_exec<'a>(
        channel_id: Channel,
        src: ThinString<'a>,
        output: Boolean,
        err: *mut Error,
    );
}
