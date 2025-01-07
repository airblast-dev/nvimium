use nvim_types::{array::Array, dictionary::Dictionary, error::Error, func_types::KeyMapMode, string::ThinString};

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
    pub fn nvim_del_var<'a>(var_name:ThinString<'a>, err: *const Error);
    // Array<Array<[String; 2]>>
    pub fn nvim_echo<'a>(chunks: Array, history: bool, opts: Dictionary);
    pub fn nvim_err_write<'a>(s: ThinString<'a>);
    pub fn nvim_err_writeln<'a>(s: ThinString<'a>);
}
