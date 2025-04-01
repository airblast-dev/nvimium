use crate::{Boolean, string::OwnedThinString};

#[derive(Clone, Debug)]
pub struct Mode {
    pub mode: OwnedThinString,
    pub blocking: Boolean,
}
