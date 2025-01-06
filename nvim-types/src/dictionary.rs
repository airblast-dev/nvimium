use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{kvec::KVec, object::Object, string::String};

#[repr(C)]
pub struct DictKVec {
    key: String,
    object: Object,
}

#[repr(transparent)]
pub struct Dictionary(KVec<DictKVec>);

impl Deref for Dictionary {
    type Target = [DictKVec];
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
        String: PartialEq<K>,
    {
        self.deref().iter().find_map(
            |DictKVec { key: k, object: o }| {
                if *k == key {
                    Some(o)
                } else {
                    None
                }
            },
        )
    }

    pub fn insert<K>(&mut self, key: K, mut object: Object) -> Option<Object>
    where
        String: PartialEq<K> + From<K>,
    {
        for DictKVec { key: k, object: o } in self.0.iter_mut() {
            if *k == key {
                std::mem::swap(o, &mut object);
                return Some(object);
            }
        }

        self.0.push(DictKVec {
            key: String::from(key),
            object,
        });

        None
    }
}

#[repr(C)]
pub struct TypedDictionary<B> {
    inner: KVec<DictKVec>,
    __p: PhantomData<B>,
}

impl<B> Deref for TypedDictionary<B> {
    type Target = [DictKVec];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<B> DerefMut for TypedDictionary<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

const _: () = assert!(24 == std::mem::size_of::<Dictionary>());

#[cfg(test)]
mod dict {
    #[test]
    fn get() {}
}
