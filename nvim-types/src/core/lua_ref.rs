use super::LuaRefT;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct LuaRef(LuaRefT);
