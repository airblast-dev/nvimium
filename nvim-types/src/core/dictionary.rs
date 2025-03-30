use std::ops::{Deref, DerefMut};

use crate::{kvec::KVec, object::Object, string::String};

use super::{borrowed::Borrowed, string::OwnedThinString};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct KeyValuePair {
    pub key: OwnedThinString,
    pub object: Object,
}

impl Clone for KeyValuePair {
    fn clone(&self) -> Self {
        KeyValuePair {
            key: self.key.clone(),
            object: self.object.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.object.clone_from(&source.object);
        self.key.clone_from(&source.key);
    }
}

impl From<(String, Object)> for KeyValuePair {
    fn from((key, object): (String, Object)) -> Self {
        Self {
            key: OwnedThinString::from(key),
            object,
        }
    }
}

impl From<(Object, String)> for KeyValuePair {
    fn from((object, key): (Object, String)) -> Self {
        Self {
            key: OwnedThinString::from(key),
            object,
        }
    }
}

#[repr(transparent)]
#[derive(Default, Debug, PartialEq)]
pub struct Dictionary(pub KVec<KeyValuePair>);

impl Clone for Dictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

impl Deref for Dictionary {
    type Target = [KeyValuePair];
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for Dictionary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Dictionary {
    pub fn get<K>(&self, key: K) -> Option<&Object>
    where
        K: PartialEq<OwnedThinString>,
    {
        let index = self.find_by_key(&key)?;
        unsafe { Some(&self.0.as_slice().get_unchecked(index).object) }
    }

    pub fn remove<K>(&mut self, key: K) -> Option<KeyValuePair>
    where
        K: PartialEq<OwnedThinString>,
    {
        let index = self.find_by_key(&key)?;
        Some(self.0.swap_remove(index))
    }

    pub fn remove_skip_key_drop<K>(&mut self, key: K) -> Option<Object>
    where
        K: PartialEq<OwnedThinString>,
    {
        let index = self.find_by_key(&key)?;

        let KeyValuePair { key, object } = self.0.swap_remove(index);
        core::mem::forget(key);
        Some(object)
    }

    pub fn insert<K>(&mut self, key: K, mut object: Object) -> Option<Object>
    where
        K: PartialEq<OwnedThinString>,
        OwnedThinString: From<K>,
    {
        let index = self.find_by_key(&key);
        match index {
            Some(index) => {
                core::mem::swap(&mut object, unsafe {
                    &mut self.0.get_unchecked_mut(index).object
                });
                Some(object)
            }
            None => {
                self.0.push(KeyValuePair {
                    key: OwnedThinString::from(key),
                    object,
                });
                None
            }
        }
    }

    /// Returns the index for a key if the key is present
    ///
    /// The returned index is guaranteed to be the index to the key value pair.
    fn find_by_key<K>(&self, key: &K) -> Option<usize>
    where
        K: PartialEq<OwnedThinString>,
    {
        self.0
            .iter()
            .position(|KeyValuePair { key: k, .. }| key == k)
    }
}

impl From<&[KeyValuePair]> for Dictionary {
    fn from(value: &[KeyValuePair]) -> Self {
        Self(KVec::from(value))
    }
}

impl<KV> FromIterator<KV> for Dictionary
where
    KV: Clone + Into<KeyValuePair>,
{
    fn from_iter<T: IntoIterator<Item = KV>>(iter: T) -> Self {
        Self(KVec::from_iter(iter.into_iter().map(KV::into)))
    }
}

const _: () = assert!(24 == std::mem::size_of::<Dictionary>());

impl<'a> From<&'a Dictionary> for Borrowed<'a, Dictionary> {
    fn from(value: &'a Dictionary) -> Self {
        Borrowed::new(value)
    }
}

#[cfg(test)]
mod dict {

    #[test]
    fn get() {}
}
