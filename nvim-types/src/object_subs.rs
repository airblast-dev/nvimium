use macros::one_of_objects;

use crate::{Integer, object::Object, string::OwnedThinString};

one_of_objects! {
    #[doc(hidden)]
    #[derive(Debug)]
    pub StringOrInt,
    OwnedThinString,
    Integer
}
