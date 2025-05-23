use std::mem::ManuallyDrop;

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
            mode: if matches!(*mode, Object::String(_)) {
                ManuallyDrop::into_inner(mode).into_string().unwrap()
            } else {
                panic!()
            },
            blocking: if matches!(*blocking, Object::Bool(_)) {
                ManuallyDrop::into_inner(blocking).into_bool().unwrap()
            } else {
                panic!()
            },
        }
    }
}
