use core::marker::PhantomData;

use super::LuaRefT;

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LuaRef(LuaRefT, PhantomData<*mut LuaRefT>);

impl LuaRef {
    /// Initialize a new LuaRef with an key
    ///
    /// # Safety
    ///
    /// Passing an invalid key can result in UB in some cases.
    pub unsafe fn new(key: LuaRefT) -> Self {
        Self(key, PhantomData)
    }
    pub fn as_int(&self) -> LuaRefT {
        self.0
    }
}
