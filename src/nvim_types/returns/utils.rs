use std::{ffi::CStr, mem::ManuallyDrop};

use crate::nvim_types::{Dict, Object};

#[inline(never)]
pub(crate) fn remove_keys(
    keys: &'static [&'static CStr],
    d: &mut Dict,
    cleanup: Option<fn([ManuallyDrop<Object>; 20])>,
) -> Result<[ManuallyDrop<Object>; 20], &'static CStr> {
    let len = keys.len();
    let mut i = 0;
    // an array length of 20 should enough for all return values
    let mut objects = [const { ManuallyDrop::new(Object::Null) }; 20];
    // based on the std libraries comments this is unlikely to unroll which is good for binary size
    while i < len {
        let key = keys[i];
        let Some(val) = d.remove_skip_key_drop(key) else {
            if let Some(f) = cleanup {
                f(objects);
            }
            return Err(key);
        };

        objects[i] = ManuallyDrop::new(val);
        i += 1;
    }

    Ok(objects)
}
