use crate::one_of_objects;

use super::{object::Object, string::OwnedThinString, Integer, LuaRef};

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

impl Default for LuaRefOrString {
    fn default() -> Self {
        Self::from(OwnedThinString::default())
    }
}
