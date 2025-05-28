use thread_lock::call_check;

use crate::{
    nvim_funcs::c_funcs::command::{nvim_buf_create_user_command, nvim_buf_del_user_command},
    nvim_types::{
        AsThinString, Buffer, Channel, Error, ThinString, func_types::create_user_command::Command,
        opts::create_user_command::CreateUserCommandOpts,
    },
    tri,
};

pub fn buf_create_user_command<'a, C: Into<Command<'a>>>(
    buf: Buffer,
    name: ThinString<'a>,
    command: C,
    opts: &mut CreateUserCommandOpts<'a>,
) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {nvim_buf_create_user_command(Channel::LUA_INTERNAL_CALL, buf, name, command.into(), opts, &mut err);}
    }
}

pub fn buf_del_user_command<TH: AsThinString>(buf: Buffer, name: TH) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {nvim_buf_del_user_command(buf, name.as_thinstr(), &mut err);}
    }
}

mod tests {
    use crate::{
        nvim_funcs::vimscript::exec2,
        nvim_types::{
            AsThinString, Buffer,
            opts::{
                create_user_command::{
                    CreateUserCommandOpts, UserCommandAddr, UserCommandCompleteKind,
                    UserCommandNarg,
                },
                exec::ExecOpts,
            },
        },
    };

    use super::buf_create_user_command;

    fn buf_create_del_user_command() {
        buf_create_user_command(
            Buffer::new(0),
            c"MyCmdNvimium".as_thinstr(),
            c"echoerr".as_thinstr(),
            CreateUserCommandOpts::default()
                .complete(UserCommandCompleteKind::MESSAGES)
                .nargs(UserCommandNarg::ONE)
                .force(true)
                .addr(UserCommandAddr::ARGUMENTS),
        )
        .unwrap();

        exec2(c"MyCmdNvimium", &ExecOpts::default()).unwrap_err();
        exec2(c"MyCmdNvimium", &ExecOpts::default()).unwrap_err();
    }
}
