use std::{error::Error, mem::ManuallyDrop};

use crate::nvim_types::{
    AsThinString,
    args::user_command::UserCommandArgs,
    lua::Function,
    object::{ObjectRef, ObjectTag},
};

#[repr(transparent)]
pub struct UserCommand<'a>(ObjectRef<'a>);

impl<'a> UserCommand<'a> {
    pub fn command<TH: AsThinString>(cmd: &'a TH) -> Self {
        Self(ObjectRef::from(cmd.as_thinstr()))
    }
}

impl UserCommand<'static> {
    pub fn callback<
        E: 'static + Error,
        F: 'static + for<'f> Fn(UserCommandArgs<'f>) -> Result<(), E> + Unpin,
    >(
        cmd: F,
    ) -> Self {
        UserCommand(ObjectRef::from(Function::wrap(cmd).into_luaref()))
    }
}

// Command only takes a thinstring or a LuaRef
// the LuaRef is owned so we must unref it to avoid a leak
impl<'a> Drop for UserCommand<'a> {
    fn drop(&mut self) {
        if self.0.tag == ObjectTag::LuaRef {
            unsafe { ManuallyDrop::drop(&mut self.0.val.lua_ref) };
        }
    }
}
