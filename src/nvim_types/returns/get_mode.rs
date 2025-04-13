use std::mem::ManuallyDrop;

use crate::nvim_types::{string::OwnedThinString, Boolean, Dict, Object};

use super::utils::remove_keys;

#[derive(Clone, Debug)]
pub struct Mode {
    pub mode: OwnedThinString,
    pub blocking: Boolean,
}

impl Mode {
    pub fn from_c_func_ret(d: &mut Dict) -> Self {
        let [mode, blocking, ..] = remove_keys(&[c"mode", c"blocking"], d, None).unwrap();
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
            }
        }
    }
}
