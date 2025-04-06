use core::{
    marker::PhantomData,
    sync::atomic::{AtomicPtr, Ordering},
};
use std::ffi::c_void;

use libc::{bsearch, c_char, c_int, qsort, strcmp};

use crate::nvim_types::{nvalloc::xmalloc, string::AsThinString};

use crate::nvim_types::{
    dictionary::{Dictionary, KeyValuePair},
    kvec::KVec,
    string::ThinString,
};

/// The color values returned by neovim are static, this means we can only allocate once and just
/// reuse it when reading from it.
///
/// This allows to avoid extra compute and memory usage. This also allows us to perform faster
/// lookups as we store the colors sorted by their names which allows for a binary search.
/// (neovim actually returns the values sorted as is but this isn't a guarantee so we ensure that
/// it is sorted ourselves)
static COLOR_MAP: AtomicPtr<KVec<(ThinString<'static>, [u8; 3])>> =
    AtomicPtr::new(core::ptr::null_mut());

/// Neovim's color map
///
/// Often used when creating a Highlight group.
#[derive(Clone, Debug)]
pub struct ColorMap {
    /// Marker only used to mark to dissallow initialization from external sources.
    p: PhantomData<&'static ()>,
}

impl ColorMap {
    /// Initializes a [`ColorMap`] by draining the values out of the dictonary
    ///
    /// # Note
    /// The [`Drop`] impl for the [`Dictionary`]'s key values are not called because the strings
    /// for color names are stored in constants.
    ///
    /// This function is guaranteed to drain all values stored in the [`Dictionary`].
    pub fn from_c_func_ret(d: &mut Dictionary) -> Self {
        let mut kv = KVec::with_capacity(d.len());
        for i in (0..d.len()).rev() {
            // TODO: replace with pop
            let KeyValuePair {
                key: color_name,
                object: color_value,
            } = d.0.swap_remove(i);
            let name = color_name.leak();
            let Some(value) = color_value.clone().into_int() else {
                // should be impossible to reach but better than a panic
                continue;
            };
            let rgb = [(value >> 16) as u8, (value >> 8) as u8, value as u8];
            kv.push((name, rgb));
        }

        if COLOR_MAP.load(Ordering::SeqCst).is_null() {
            #[cfg(miri)]
            kv.as_mut_slice()
                .sort_unstable_by(|c1, c2| c1.partial_cmp(c2).expect("non ascii color name"));
            let kv_ptr = unsafe { xmalloc(size_of::<KVec<(ThinString<'static>, [u8; 3])>>(), 1) }
                .as_ptr() as *mut KVec<(ThinString<'static>, [u8; 3])>;
            // shrinks binary size by 6kb depending on the project
            #[cfg(not(miri))]
            {
                unsafe {
                    qsort(
                        kv.as_mut_ptr() as *mut c_void,
                        kv.len(),
                        size_of::<(ThinString, [u8; 3])>(),
                        Some(qs_th),
                    );
                };
                unsafe extern "C" fn qs_th(p1: *const c_void, p2: *const c_void) -> c_int {
                    unsafe {
                        let a = (p1 as *const (ThinString, [u8; 3]))
                            .as_ref()
                            .unwrap_unchecked();
                        let b = (p2 as *const (ThinString, [u8; 3]))
                            .as_ref()
                            .unwrap_unchecked();
                        strcmp(a.0.as_ptr() as *const c_char, b.0.as_ptr() as *const c_char)
                    }
                }
            }
            unsafe { kv_ptr.write(kv) };
            COLOR_MAP.store(kv_ptr, Ordering::SeqCst);
        }
        Self::initialized()
    }

    /// Returns `true` if a color map is currently loaded.
    ///
    /// If the return value is true [`ColorMap::initialized`] is guaranteed to be panic free.
    pub fn is_loaded() -> bool {
        !COLOR_MAP.load(Ordering::SeqCst).is_null()
    }

    /// Returns an initialized [`ColorMap`]
    ///
    /// # Panics
    ///
    /// If the internal color map is not initialized via [`ColorMap::from_c_func_ret`]
    pub fn initialized() -> Self {
        assert!(Self::is_loaded());
        Self { p: PhantomData }
    }

    pub fn get_with_name<N>(&self, name: N) -> Option<[u8; 3]>
    where
        N: AsThinString,
    {
        // SAFETY: callers can only call this method if the color map is initialized
        let map = unsafe { COLOR_MAP.load(Ordering::SeqCst).as_ref().unwrap_unchecked() };
        // validate that name is something we can compare against
        // else return None early
        let key = name.as_thinstr();
        #[cfg(not(miri))]
        {
            let key = (key, [0; 3]);
            let item = unsafe {
                bsearch(
                    &raw const key as *const c_void,
                    map.as_ptr() as *const c_void,
                    map.len(),
                    size_of::<(ThinString, [u8; 3])>(),
                    Some(bs),
                )
            };
            unsafe extern "C" fn bs(p1: *const c_void, p2: *const c_void) -> c_int {
                let p1 = unsafe {
                    (p1 as *const (ThinString, [u8; 3]))
                        .as_ref()
                        .unwrap_unchecked()
                        .0
                };
                let p2 = unsafe {
                    (p2 as *const (ThinString, [u8; 3]))
                        .as_ref()
                        .unwrap_unchecked()
                        .0
                };
                unsafe { strcmp(p1.as_ptr() as *const c_char, p2.as_ptr() as *const c_char) }
            }
            if item.is_null() {
                None
            } else {
                Some(
                    unsafe {
                        (item as *const (ThinString, [u8; 3]))
                            .as_ref()
                            .unwrap_unchecked()
                    }
                    .1,
                )
            }
        }
        #[cfg(miri)]
        {
            ThinString::from_null_terminated(c"".to_bytes_with_nul()).partial_cmp(&key)?;

            let idx = map
                .binary_search_by(|(s1, _)| {
                    // cannot panic validated above
                    s1.partial_cmp(&key).unwrap()
                })
                .ok()?;
            Some(map[idx].1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nvim_types::{
        dictionary::Dictionary,
        object::Object,
        string::{OwnedThinString, String},
    };

    use super::ColorMap;

    #[test]
    fn color_map_from_c_func_ret() {
        let mut dict = Dictionary::default();
        let colors: [(OwnedThinString, Object); 3] =
            [("red", 255 << 16), ("green", 255 << 8), ("blue", 255)]
                .map(|(s, c)| (OwnedThinString::from(String::from(s)), Object::Integer(c)));
        for (name, val) in colors {
            dict.insert(name, val);
        }

        let c_map = ColorMap::from_c_func_ret(&mut dict);
        drop(dict);
        assert_eq!(Some([255, 0, 0]), c_map.get_with_name(String::from("red")));
        assert_eq!(
            Some([0, 255, 0]),
            c_map.get_with_name(String::from("green"))
        );
        assert_eq!(Some([0, 0, 255]), c_map.get_with_name(String::from("blue")));
    }
}
