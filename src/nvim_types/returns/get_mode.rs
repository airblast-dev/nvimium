use std::ops::Deref;

use crate::nvim_types::{Boolean, Dict, Object, string::OwnedThinString};

use super::utils::skip_drop_remove_keys;

#[derive(Clone, Debug)]
pub struct Mode {
    pub mode: OwnedThinString,
    pub blocking: Boolean,
}

impl Mode {
    pub fn from_c_func_ret(d: &mut Dict) -> Self {
        let [mode, blocking] = skip_drop_remove_keys(d, &["mode", "blocking"], None).unwrap();
        Self {
            mode: if let Object::String(s) = mode.deref() {
                s.clone()
            } else {
                panic!()
            },
            blocking: if let Object::Bool(b) = blocking.deref() {
                *b
            } else {
                panic!()
            },
        }
    }
}
