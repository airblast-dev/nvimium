use crate::nvim_types::{
    Buffer, Channel, Error, ThinString, args::create_user_command::Command,
    opts::create_user_command::CreateUserCommandOpts,
};

unsafe extern "C" {
    pub fn nvim_buf_create_user_command<'a>(
        chan: Channel,
        buf: Buffer,
        name: ThinString<'a>,
        command: Command<'a>,
        opts: CreateUserCommandOpts<'a>,
        err: *mut Error,
    );

    pub fn nvim_buf_del_user_command<'a>(buf: Buffer, name: ThinString<'a>, err: *mut Error);
}
