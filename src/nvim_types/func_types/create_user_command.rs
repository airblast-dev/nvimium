use std::mem::ManuallyDrop;

use crate::nvim_types::{
    object::{ObjectRef, ObjectTag}, AsThinString, ThinString
};

#[repr(transparent)]
pub struct UserCommand<'a>(ObjectRef<'a>);

impl<'a> UserCommand<'a> {
    pub(crate) fn cmd(cmd: ThinString<'a>) -> Self {
        Self(ObjectRef::from(cmd))
    }
    // TODO: add lua callback support
    fn callback() {}
}

impl<'a, TH: AsThinString> From<&'a TH> for UserCommand<'a> {
    fn from(value: &'a TH) -> Self {
        Self::cmd(value.as_thinstr())
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
