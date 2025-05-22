use std::{mem::MaybeUninit, ptr::NonNull};

use crate::nvim_types::{Dict, Object};

/// Removes the keys without dropping them and
#[inline(never)]
pub(crate) unsafe fn remove_keys(
    len: usize,
    mut keys: NonNull<&'static str>,
    mut out: NonNull<MaybeUninit<Object>>,
    d: &mut Dict,
    missing_key: Option<fn(&'static str) -> Option<Object>>,
) -> Result<(), &'static str> {
    let mut i = 0;
    // an array length of 20 should enough for all return values
    // based on the std libraries comments this is unlikely to unroll which is good for binary size
    while i < len {
        let key = unsafe { keys.read() };
        let Some(val) = d
            .remove_skip_key_drop(key)
            .or_else(|| missing_key.and_then(|f| f(key)))
            .map(MaybeUninit::new)
        else {
            return Err(key);
        };
        unsafe {
            out.write(val);
            keys = keys.add(1);
            out = out.add(1);
        };

        i += 1;
    }

    Ok(())
}

macro_rules! extract_owned_kv {
    (
        $collector:ident,
        $s:ident, $matches:pat $(, $extract:expr)? $(, $fallback:expr)?
    ) => {
        if let Some($matches) = d.remove_skip_key_drop(stringify!($s)) {
            $extract($ident);
        } else {
            $fallback($ident);
        }
    };
}

macro_rules! extract_ref_kv {
    (
        $collector:ident,
        $s:ident, $matches:pat $(, $extract:expr)? $(, $fallback:expr)?
    ) => {
        if let Some($matches) = d
            .remove_skip_key_drop(stringify!($s))
            .map(::std::mem::ManuallyDrop::new)
            .deref()
        {
            $extract($ident);
        } else {
            $fallback($ident);
        }
    };
}
