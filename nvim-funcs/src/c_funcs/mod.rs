use nvim_types::{
    array::Array,
    error::Error,
    func_types::KeyMapMode,
    opts::{echo::EchoOpts, eval_statusline::EvalStatusLineOpts},
    string::ThinString,
    Arena,
};

extern "C" {
    pub fn nvim_create_buf(listed: bool, scratch: bool);
    pub fn nvim_del_current_line();
    pub fn nvim_del_keymap<'a>(
        map_mode: KeyMapMode,
        lhs: ThinString<'a>,
        error: ThinString<'a>,
        err: *const Error,
    );
    pub fn nvim_del_mark<'a>(name: ThinString<'a>, err: *const Error);
    pub fn nvim_del_var<'a>(var_name: ThinString<'a>, err: *const Error);
    // Array<Array<[String; 2]>> replace opts dict
    pub fn nvim_echo<'a>(chunks: Array, history: bool, opts: *const EchoOpts);
    pub fn nvim_err_write<'a>(s: ThinString<'a>);
    pub fn nvim_err_writeln<'a>(s: ThinString<'a>);
    pub fn nvim_eval_statusline<'a>(
        s: ThinString<'a>,
        opts: *const EvalStatusLineOpts<'a>,
        arena: *mut Arena,
        err: *mut Error,
    );
}
