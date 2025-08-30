use core::ops::{Deref, DerefMut};

use super::{kvec::KVec, object::Object};

use super::{borrowed::Borrowed, string::OwnedThinString};

/// A key value pair to be stored in a [`Dict`]
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

impl<S> From<(S, Object)> for KeyValuePair
where
    OwnedThinString: From<S>,
{
    fn from((key, object): (S, Object)) -> Self {
        Self {
            key: OwnedThinString::from(key),
            object,
        }
    }
}

impl<S> From<(Object, S)> for KeyValuePair
where
    OwnedThinString: From<S>,
{
    fn from((object, key): (Object, S)) -> Self {
        Self {
            key: OwnedThinString::from(key),
            object,
        }
    }
}

#[repr(transparent)]
#[derive(Default, Debug)]
pub struct Dict(pub(crate) KVec<KeyValuePair>);

impl Clone for Dict {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

impl Deref for Dict {
    type Target = [KeyValuePair];
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for Dict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Dict {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.0.iter().all(|kv| other.0.contains(kv))
    }
}

impl Dict {
    /// Get the [`Object`] for the provided key.
    pub fn get<K>(&self, key: K) -> Option<&Object>
    where
        K: PartialEq<OwnedThinString>,
    {
        let index = self.find_by_key(&key)?;
        unsafe { Some(&self.0.as_slice().get_unchecked(index).object) }
    }

    /// Remove a [`KeyValuePair`] from the [`Dict`]
    pub fn remove<K>(&mut self, key: K) -> Option<KeyValuePair>
    where
        K: PartialEq<OwnedThinString>,
    {
        let index = self.find_by_key(&key)?;
        Some(self.0.swap_remove(index))
    }

    /// Remove the [`Object`] for the provided key
    ///
    /// This will intentionally not drop the key value, mainly used for returned static strings
    /// from neovim.
    pub(crate) fn remove_skip_key_drop<K>(&mut self, key: K) -> Option<Object>
    where
        K: PartialEq<OwnedThinString>,
    {
        let index = self.find_by_key(&key)?;

        let KeyValuePair { key, object } = self.0.swap_remove(index);
        core::mem::forget(key);
        Some(object)
    }

    /// Insert a new [`KeyValuePair`] into the [`Dict`]
    ///
    /// If the key is already stored in the dictionary the provided key is not used, instead the
    /// existing key will be reused and the existing [`Object`] is returned.
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
    /// The returned index is guaranteed to be the index to the key value pair with the provided
    /// key.
    fn find_by_key<K>(&self, key: &K) -> Option<usize>
    where
        K: PartialEq<OwnedThinString>,
    {
        self.0
            .iter()
            .position(|KeyValuePair { key: k, .. }| key == k)
    }

    #[inline(always)]
    pub fn into_kvec(self) -> KVec<KeyValuePair> {
        self.0
    }
}

impl From<&[KeyValuePair]> for Dict {
    fn from(value: &[KeyValuePair]) -> Self {
        Self(KVec::from(value))
    }
}

impl<KV> FromIterator<KV> for Dict
where
    KV: Clone + Into<KeyValuePair>,
{
    fn from_iter<T: IntoIterator<Item = KV>>(iter: T) -> Self {
        Self(KVec::from_iter(iter.into_iter().map(KV::into)))
    }
}

const _: () = assert!(24 == core::mem::size_of::<Dict>());

impl<'a> From<&'a Dict> for Borrowed<'a, Dict> {
    fn from(value: &'a Dict) -> Self {
        Borrowed::new(value)
    }
}

impl From<KVec<KeyValuePair>> for Dict {
    fn from(value: KVec<KeyValuePair>) -> Self {
        Self(value)
    }
}
