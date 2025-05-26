use crate::one_of_objects;

use super::{object::{Object, ObjectTag}, string::OwnedThinString, Boolean, Integer, LuaRef, ThinString};

one_of_objects! {
    #[doc(hidden)]
    #[derive(Clone, Debug)]
    pub StringOrInt,
    OwnedThinString,
    Integer
}

one_of_objects! {
    #[doc(hidden)]
    #[derive(Clone, Debug, PartialEq)]
    pub LuaRefOrString,
    OwnedThinString,
    LuaRef
}

one_of_objects! {
    #[doc(hidden)]
    #[derive(Clone, Debug, PartialEq)]
    pub BoolOrInteger,
    Boolean,
    Integer
}

impl Default for LuaRefOrString {
    fn default() -> Self {
        Self::from(OwnedThinString::default())
    }
}

#[repr(C, u32)]
pub enum ThinStringOrBool<'a> {
    String(ThinString<'a>) = ObjectTag::String as u32,
    Bool(Boolean) = ObjectTag::Bool as u32,
}
