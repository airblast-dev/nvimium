use super::LuaRefT;

#[repr(transparent)]
#[derive(Debug)]
pub struct LuaRef(LuaRefT);

impl LuaRef {
    /// Initialize a new LuaRef with an key
    ///
    /// # Safety
    ///
    /// Passing an invalid key can result in UB in some cases.
    pub unsafe fn new(key: LuaRefT) -> Self {
        Self(key)
    }
    pub fn as_int(&self) -> LuaRefT {
        self.0
    }
}
