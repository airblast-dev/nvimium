use core::ops::{Deref, DerefMut};

use super::{KVec, Object};

use super::borrowed::Borrowed;

/// A [`KVec`] of [`Object`]s
///
/// The implementation intentionally does not provide methods on [`Array`] itself and delegates the
/// implementations via [`Deref`] to [`KVec`]. See its documentation instead.
#[repr(transparent)]
#[derive(Default, Debug, PartialEq)]
pub struct Array(pub KVec<Object>);

impl Clone for Array {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(source);
    }
}

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

impl From<&[Object]> for Array {
    fn from(value: &[Object]) -> Self {
        Self(KVec::from(value))
    }
}

impl Array {
    pub(crate) fn into_kvec(self) -> KVec<Object> {
        self.0
    }
}

impl<'a> From<&'a Array> for Borrowed<'a, Array> {
    fn from(value: &'a Array) -> Self {
        Borrowed::new(value)
    }
}
