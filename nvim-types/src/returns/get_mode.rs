use crate::{string::OwnedThinString, Boolean};

#[derive(Clone, Debug)]
pub struct Mode {
    pub mode: OwnedThinString,
    pub blocking: Boolean,
}
