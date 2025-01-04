use std::marker::PhantomData;

use crate::{kvec::KVec, object::Object};

#[repr(C)]
pub struct DictKVec {
    key: String,
    object: Object,
}

#[repr(transparent)]
pub struct Dictionary(KVec<DictKVec>);

#[repr(C)]
struct KeyValPair(String, Object);

#[repr(C)]
pub struct TypedDictionary<T, B> {
    inner: KVec<KeyValPair>,
    __p: PhantomData<(T, B)>
}

const _: () = assert!(24 == std::mem::size_of::<Dictionary>());
