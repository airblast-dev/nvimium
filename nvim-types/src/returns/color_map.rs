use std::{marker::PhantomData, sync::OnceLock};

use crate::{
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
static COLOR_MAP: OnceLock<KVec<(ThinString<'static>, [u8; 3])>> = OnceLock::new();

/// A
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
            let Some(value) = color_value.clone().as_int() else {
                // should be impossible to reach but better than a panic
                continue;
            };
            let rgb = [(value >> 16) as u8, (value >> 8) as u8, value as u8];
            kv.push((name, rgb));
        }

        kv.as_mut_slice()
            .sort_unstable_by(|c1, c2| c1.partial_cmp(c2).expect("non ascii color name"));
        let _ = COLOR_MAP.set(kv);
        Self::initialized()
    }

    /// Returns `true` if a color map is currently loaded.
    ///
    /// If the return value is true [`ColorMap::initialized`] is guaranteed to be panic free.
    pub fn is_loaded() -> bool {
        COLOR_MAP.get().is_some()
    }

    /// Returns an initialized [`ColorMap`]
    ///
    /// # Panics
    ///
    /// If the internal color map is not initialized via [`ColorMap::from_c_func_ret`]
    pub fn initialized() -> Self {
        assert!(COLOR_MAP.get().is_some());
        Self { p: PhantomData }
    }

    pub fn get_with_name<'a, N>(&self, name: N) -> Option<[u8; 3]>
    where
        ThinString<'a>: PartialOrd<N>,
    {
        let map = COLOR_MAP
            .get()
            .expect("uninitialized ColorMap, this is most likely an internal bug inside nvimium");
        // validate that name is something we can compare against
        // else return None early
        ThinString::from_null_terminated(b"").partial_cmp(&name)?;
        let idx = map
            .binary_search_by(|(s1, _)| {
                // cannot panic validated above
                s1.partial_cmp(&name).unwrap()
            })
            .ok()?;
        Some(map[idx].1)
    }
}
