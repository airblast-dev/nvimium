use thread_lock::call_check;

use crate::{
    nvim_funcs::c_funcs::command::{nvim_buf_create_user_command, nvim_buf_del_user_command},
    nvim_types::{
        AsThinString, Buffer, Channel, Error, ThinString, func_types::create_user_command::Command,
        opts::create_user_command::CreateUserCommandOpts,
    },
    tri,
};

pub fn buf_create_user_command<'a>(
    chan: Channel,
    buf: Buffer,
    name: ThinString<'a>,
    command: Command<'a>,
    opts: CreateUserCommandOpts<'a>,
) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {nvim_buf_create_user_command(chan, buf, name, command, opts, &mut err);}
    }
}

pub fn buf_del_user_command<TH: AsThinString>(buf: Buffer, name: TH) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {nvim_buf_del_user_command(buf, name.as_thinstr(), &mut err);}
    }
}
