use std::mem::ManuallyDrop;

use crate::nvim_types::{Dict, Object, OwnedThinString};

use super::utils::skip_drop_remove_keys;

pub struct Exec2 {
    pub output: Option<OwnedThinString>,
}

impl Exec2 {
    pub(crate) fn from_c_func_ret(d: &mut Dict) -> Self {
        let [output] = skip_drop_remove_keys(d, &["output"], Some(|_| Some(Object::Null))).unwrap();
        if !matches!(&*output, Object::Null | Object::String(_)) {
            panic!("exec2 output unknown type");
        }
        Self {
            output: ManuallyDrop::into_inner(output).into_string(),
        }
    }
}
