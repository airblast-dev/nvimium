use std::ops::{Deref, DerefMut};

use crate::{kvec::KVec, object::Object};

/// A [`KVec`] of [`Object`]s
///
/// The implementation intentionally does not provide methods on [`Array`] itself and delegates the
/// implementations via [`Deref`] to [`KVec`]. See its documentation instead.
#[repr(transparent)]
pub struct Array(KVec<Object>);

impl Deref for Array {
    type Target = KVec<Object>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
