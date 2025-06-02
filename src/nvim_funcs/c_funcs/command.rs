use std::mem::MaybeUninit;

use crate::nvim_types::{
    Arena, Buffer, Channel, Dict, Error, ThinString,
    func_types::create_user_command::Command,
    opts::{create_user_command::CreateUserCommandOpts, get_commands::GetCommandOpts},
};

unsafe extern "C" {
    pub fn nvim_buf_create_user_command<'a>(
        chan: Channel,
        buf: Buffer,
        name: ThinString<'a>,
        command: Command<'a>,
        opts: *mut CreateUserCommandOpts<'a>,
        err: *mut Error,
    );

    pub fn nvim_buf_del_user_command<'a>(buf: Buffer, name: ThinString<'a>, err: *mut Error);
    pub fn nvim_buf_get_commands(
        buf: Buffer,
        opts: *mut GetCommandOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dict>;
}
