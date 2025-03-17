use macros::one_of_objects;

use crate::{object::Object, string::OwnedThinString, Integer};

one_of_objects! {
    #[doc(hidden)]
    #[derive(Debug)]
    pub StringOrInt,
    OwnedThinString,
    Integer
}
