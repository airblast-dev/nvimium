use crate::nvim_types::{
    array::Array,
    borrowed::Borrowed,
    hl_group::HlGroupId,
    kvec::KVec,
    object::Object,
    string::{OwnedThinString, ThinString},
};

#[repr(transparent)]
pub struct Echo(Array);

// TODO: rewrite with less alocator trips
impl<'a> FromIterator<(ThinString<'a>, Option<HlGroupId>)> for Echo {
    fn from_iter<T: IntoIterator<Item = (ThinString<'a>, Option<HlGroupId>)>>(iter: T) -> Self {
        let top_k: KVec<Object> = KVec::from_iter(iter.into_iter().map(|(th, hl)| {
            let mut k = KVec::with_capacity(1 + (hl.is_some() as usize));
            k.push(Object::String(OwnedThinString::from(th)));
            if let Some(hl) = hl {
                k.push(Object::Integer(hl.as_int()));
            }
            Object::Array(Array(k))
        }));

        Echo(Array(top_k))
    }
}

impl Echo {
    pub fn message<S>(th: S) -> Self
    where
        OwnedThinString: From<S>,
    {
        Self::msg(OwnedThinString::from(th))
    }

    pub(crate) fn msg(s: OwnedThinString) -> Self {
        let mut kv = KVec::with_capacity(1);
        let mut inner_kv = KVec::with_capacity(1);
        inner_kv.push(Object::String(s));
        kv.push(Object::Array(Array(inner_kv)));
        Self(Array(kv))
    }
}

impl<'a> From<&'a Echo> for Borrowed<'a, Echo> {
    fn from(value: &'a Echo) -> Self {
        Borrowed::new(value)
    }
}
