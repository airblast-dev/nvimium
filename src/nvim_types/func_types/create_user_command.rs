use crate::nvim_types::{
    AsThinString, LuaRef,
    object::{ObjectRef, ObjectTag},
};

#[repr(transparent)]
pub struct Command<'a>(ObjectRef<'a>);

impl<'a> Command<'a> {
    pub fn cmd<TH: AsThinString>(cmd: &'a TH) -> Self {
        Self(ObjectRef::from(cmd.as_thinstr()))
    }
    // TODO: add lua callback support
    fn callback() {}
}

// Command only takes a thinstring or a LuaRef
// the LuaRef is owned so we must unref it to avoid a leak
impl<'a> Drop for Command<'a> {
    fn drop(&mut self) {
        if self.0.tag == ObjectTag::LuaRef {
            unsafe { LuaRef::new(self.0.val[0] as _) };
        }
    }
}
