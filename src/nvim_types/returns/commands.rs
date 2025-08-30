use std::ops::Deref;

use crate::nvim_types::{
    Boolean, Dict, Integer, KVec, Object, OwnedThinString,
    opts::create_user_command::UserCommandNarg,
};

use super::utils::skip_drop_remove_keys;

pub struct CommandsInfos(pub KVec<CommandInfos>);

impl CommandsInfos {
    pub(crate) fn from_c_func_ret(d: &mut Dict) -> Self {
        let mut commands = KVec::with_capacity(d.len());
        commands.extend(d.iter_mut().map(|kv| match &mut kv.object {
            Object::Dict(d) => CommandInfos::from_c_func_ret(d),
            _ => unreachable!("found non dict command mapping"),
        }));

        Self(commands)
    }
}

// TODO: use User Command types instead of strings
pub struct CommandInfos {
    pub name: OwnedThinString,
    pub definition: OwnedThinString,
    pub script_id: Integer,
    pub bang: Boolean,
    pub bar: Boolean,
    pub register: Boolean,
    pub keepscript: Boolean,
    pub preview: Boolean,
    pub nargs: UserCommandNarg,
    pub complete: Option<OwnedThinString>,
    pub complete_arg: Option<OwnedThinString>,
    pub count: Option<OwnedThinString>,
    pub range: Option<OwnedThinString>,
    pub addr: Option<OwnedThinString>,
}

impl CommandInfos {
    pub(crate) fn from_c_func_ret(d: &mut Dict) -> Self {
        let [
            name,
            definition,
            script_id,
            bang,
            bar,
            register,
            keepscript,
            preview,
            nargs,
            complete,
            complete_arg,
            count,
            range,
            addr,
        ] = skip_drop_remove_keys(
            d,
            &[
                "name",
                "definition",
                "script_id",
                "bang",
                "bar",
                "register",
                "keepscript",
                "preview",
                "nargs",
                "complete",
                "complete_arg",
                "count",
                "range",
                "addr",
            ],
            Some(|s| match s {
                "complete" | "complete_arg" | "count" | "range" | "addr" => Some(Object::Null),
                _ => None,
            }),
        )
        .unwrap();

        let name = if let Object::String(s) = name.deref() {
            s.clone()
        } else {
            unreachable!("found non string name for command");
        };

        let definition = if let Object::String(s) = definition.deref() {
            s.clone()
        } else {
            unreachable!("found non string definition for command");
        };

        let script_id = if let Object::Integer(i) = script_id.deref() {
            *i
        } else {
            unreachable!("found non integer script_id for command");
        };

        let bang = if let Object::Bool(b) = bang.deref() {
            *b
        } else {
            unreachable!("found non boolean bang for command");
        };

        let bar = if let Object::Bool(b) = bar.deref() {
            *b
        } else {
            unreachable!("found non boolean bar for command");
        };

        let register = if let Object::Bool(b) = register.deref() {
            *b
        } else {
            unreachable!("found non boolean register for command");
        };

        let keepscript = if let Object::Bool(b) = keepscript.deref() {
            *b
        } else {
            unreachable!("found non boolean keepscript for command");
        };

        let preview = if let Object::Bool(b) = preview.deref() {
            *b
        } else {
            unreachable!("found non boolean preview for command");
        };

        let nargs = if let Object::String(s) = nargs.deref() {
            match s.as_thinstr().as_slice() {
                b"0" => UserCommandNarg::ZERO,
                b"*" => UserCommandNarg::ZERO_OR_MORE,
                b"?" => UserCommandNarg::ZERO_OR_ONE,
                b"+" => UserCommandNarg::ONE_OR_MORE,
                b"1" => UserCommandNarg::ONE,
                _ => unreachable!("unknown narg value"),
            }
        } else {
            unreachable!("found non string nargs for command");
        };

        let complete = complete.deref().clone().into_string();
        let complete_arg = complete_arg.deref().clone().into_string();
        let count = count.deref().clone().into_string();
        let range = range.deref().clone().into_string();
        let addr = addr.deref().clone().into_string();

        CommandInfos {
            name,
            definition,
            script_id,
            bang,
            bar,
            register,
            keepscript,
            preview,
            nargs,
            complete,
            complete_arg,
            count,
            range,
            addr,
        }
    }
}
