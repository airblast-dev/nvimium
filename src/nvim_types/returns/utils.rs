use std::mem::{ManuallyDrop, MaybeUninit};

use crate::nvim_types::{Dict, Object};

/// Removes the keys without dropping them and
#[inline(never)]
unsafe fn remove_keys(
    keys: &[&'static str],
    out: &mut [MaybeUninit<Object>],
    d: &mut Dict,
    missing_key: fn(&'static str) -> Option<Object>,
) -> Result<(), &'static str> {
    match keys.iter().zip(out.iter_mut()).try_for_each(|(key, obj)| {
        let Some(val) = d
            .remove_skip_key_drop(*key)
            .or_else(|| missing_key(key))
            .map(MaybeUninit::new)
        else {
            return Err(*key);
        };

        *obj = val;
        Ok(())
    }) {
        Ok(()) => Ok(()),
        Err(err) => Err(err),
    }
}

pub(crate) fn skip_drop_remove_keys<const N: usize>(
    d: &mut Dict,
    keys: &'static [&'static str; N],
    missing_key: Option<fn(&'static str) -> Option<Object>>,
) -> Result<[ManuallyDrop<Object>; N], &'static str> {
    let mut objects = [const { MaybeUninit::uninit() }; N];
    unsafe {
        remove_keys(
            keys.as_slice(),
            &mut objects,
            d,
            missing_key.unwrap_or(|_| None),
        )
    }?;

    // cant use transmute with arrays in this context
    Ok(unsafe { ((&raw const objects) as *const [ManuallyDrop<Object>; N]).read() })
}
