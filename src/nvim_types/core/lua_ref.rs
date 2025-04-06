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
    /// Passing an invalid key can result in UB in some cases the exact safety requirements depend
    /// on where it will passed.
    pub const unsafe fn new(key: LuaRefT) -> Self {
        Self(key, PhantomData)
    }

    /// Get the raw integer value of the [`LuaRef`]
    pub const fn as_int(&self) -> LuaRefT {
        self.0
    }
}
