use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

use crate::dictionary::Dictionary;

#[derive(Clone, Debug)]
pub struct ColorMap {
    dict: ManuallyDrop<Dictionary>,
}

impl ColorMap {
    pub fn from_c_func_ret(d: ManuallyDrop<Dictionary>) -> Self {
        Self { dict: d }
    }
}

// cant implement DerefMut as the user may attempt to remove a key causing a free call to the const
// string
impl Deref for ColorMap {
    type Target = Dictionary;
    fn deref(&self) -> &Self::Target {
        &self.dict
    }
}

impl Drop for ColorMap {
    fn drop(&mut self) {
        // SAFETY: the dictionary stored should only have its color names as constants
        // this means we can only free the allocation for the dictionary
        unsafe { self.dict.deref_mut().0.set_len(0) };
    }
}
