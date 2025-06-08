use thread_lock::call_check;

use crate::{
    macros::tri::{tri_ez, tri_ret},
    nvim_funcs::c_funcs::command::{
        nvim_buf_create_user_command, nvim_buf_del_user_command, nvim_buf_get_commands,
        nvim_create_user_command, nvim_del_user_command, nvim_get_commands,
    },
    nvim_types::{
        AsThinString, Buffer, CALLBACK_ARENA, Channel, Error, ThinString,
        func_types::create_user_command::UserCommand,
        opts::{create_user_command::CreateUserCommandOpts, get_commands::GetCommandOpts},
        returns::commands::CommandsInfos,
    },
};

pub fn buf_create_user_command<'a>(
    buf: Buffer,
    name: ThinString<'a>,
    command: UserCommand<'a>,
    opts: &mut CreateUserCommandOpts<'a>,
) -> Result<(), Error> {
    call_check();
    tri_ez! {
        err;
        unsafe {nvim_buf_create_user_command(Channel::LUA_INTERNAL_CALL, buf, name, command, opts, &mut err)};
    }
}

pub fn buf_del_user_command<TH: AsThinString>(buf: Buffer, name: TH) -> Result<(), Error> {
    call_check();
    tri_ez! {
        err;
        unsafe {nvim_buf_del_user_command(buf, name.as_thinstr(), &mut err)};
    }
}

pub fn buf_get_commands(buf: Buffer, opts: &mut GetCommandOpts) -> Result<CommandsInfos, Error> {
    call_check();
    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_buf_get_commands(buf, opts, arena, &mut err) };
            CommandsInfos::from_c_func_ret;
        };

        arena.reset_pos();
        ret
    })
}

pub fn create_user_command<'a, TH: AsThinString>(
    name: TH,
    command: UserCommand<'a>,
    opts: &mut CreateUserCommandOpts<'a>,
) -> Result<(), Error> {
    call_check();
    tri_ez! {
        err;
        unsafe { nvim_create_user_command(Channel::LUA_INTERNAL_CALL, name.as_thinstr() , command, opts, &mut err) };
    }
}

pub fn del_user_command<TH: AsThinString>(name: TH) -> Result<(), Error> {
    call_check();
    tri_ez! {
        err;
        unsafe { nvim_del_user_command(name.as_thinstr(), &mut err) };
    }
}

pub fn get_commands(opts: &mut GetCommandOpts) -> Result<CommandsInfos, Error> {
    call_check();
    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_get_commands(opts, arena, &mut err)};
            CommandsInfos::from_c_func_ret;
        };

        arena.reset_pos();
        ret
    })
}

#[cfg(all(not(miri), feature = "testing"))]
mod tests {
    use crate as nvimium;
    use crate::nvim_funcs::global::echo;
    use crate::nvim_types::func_types::echo::Echo;
    use crate::nvim_types::opts::echo::EchoOpts;
    use crate::nvim_types::{Error, NvString, OwnedThinString};
    use crate::{
        nvim_funcs::vimscript::exec2,
        nvim_test,
        nvim_types::{
            AsThinString, Buffer,
            func_types::create_user_command::UserCommand,
            opts::{
                create_user_command::{
                    CreateUserCommandOpts, UserCommandCompleteKind, UserCommandNarg,
                },
                exec::ExecOpts,
                get_commands::GetCommandOpts,
            },
        },
    };

    use super::{
        buf_create_user_command, buf_del_user_command, buf_get_commands, create_user_command,
        del_user_command, get_commands,
    };

    #[nvim_test::nvim_test]
    fn buf_get_create_del_user_command() {
        buf_del_user_command(Buffer::new(0), c"MyCmdNvimium").unwrap_err();
        buf_create_user_command(
            Buffer::new(0),
            c"MyCmdNvimium".as_thinstr(),
            UserCommand::command(&c":echomsg \"hello\""),
            CreateUserCommandOpts::default()
                .complete(UserCommandCompleteKind::MESSAGES)
                .force(true)
                .nargs(UserCommandNarg::ZERO_OR_MORE),
        )
        .unwrap();

        exec2(c":MyCmdNvimium", &ExecOpts::default()).unwrap();

        let messages = exec2(c":messages", ExecOpts::default().output(true)).unwrap();
        assert_eq!(messages.output.unwrap(), OwnedThinString::from(c"hello"));

        let commands = buf_get_commands(Buffer::new(0), &mut GetCommandOpts::default()).unwrap();
        assert!(commands.0.iter().any(|cmd| cmd.name == c"MyCmdNvimium"));
        buf_del_user_command(Buffer::new(0), c"MyCmdNvimium").unwrap();
    }

    #[nvim_test::nvim_test]
    fn create_del_user_command() {
        del_user_command(c"MyCmd").unwrap_err();
        create_user_command(
            c"MyCmd",
            UserCommand::callback::<Error, _>(|arg| {
                let mut s = NvString::with_capacity(arg.args.len() + arg.name.len());
                write!(
                    &mut s,
                    "Called {} with argument [{}]",
                    arg.name.to_str().unwrap(),
                    arg.args.to_str().unwrap()
                )
                .unwrap();
                echo(&Echo::message(s), true, &EchoOpts::default())?;

                Ok(())
            }),
            CreateUserCommandOpts::default()
                .force(true)
                .nargs(UserCommandNarg::ONE),
        )
        .unwrap();

        let ret = exec2(c"MyCmd Hello", ExecOpts::default().output(true))
            .unwrap()
            .output
            .unwrap();
        assert_eq!("Called MyCmd with argument [Hello]", ret);
        let infos = get_commands(&mut GetCommandOpts::default()).unwrap();
        assert!(infos.0.iter().any(|info| info.name == "MyCmd"));

        del_user_command(c"MyCmd").unwrap();
    }
}
