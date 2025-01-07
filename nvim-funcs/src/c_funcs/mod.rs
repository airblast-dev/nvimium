use nvim_types::string::ThinString;

extern "C" {
    pub fn nvim_err_write<'a>(s: ThinString<'a>);
    pub fn nvim_err_writeln<'a>(s: ThinString<'a>);
    pub fn nvim_create_buf(listed: bool, scratch: bool);
    pub fn nvim_del_current_line();
}
