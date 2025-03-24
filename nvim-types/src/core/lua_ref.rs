use super::LuaRefT;

#[repr(transparent)]
#[derive(Debug)]
pub struct LuaRef(pub(crate) LuaRefT);

impl LuaRef {
    pub fn as_int(&self) -> LuaRefT {
        self.0
    }
}
