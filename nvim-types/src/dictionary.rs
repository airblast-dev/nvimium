use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{kvec::KVec, object::Object};

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

#[repr(C)]
pub struct KeyValPair(String, Object);

#[repr(C)]
pub struct TypedDictionary<B> {
    inner: KVec<KeyValPair>,
    __p: PhantomData<B>,
}

impl<B> Deref for TypedDictionary<B> {
    type Target = [KeyValPair];
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
