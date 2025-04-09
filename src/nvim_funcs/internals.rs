//! Internal neovim functions that allow some optimizations
//!
//! These functions are likely to be in the C API once officially supported.
//! The function should be niche functions but rather things we can easily implement in Rust
//! but don't for binary size.
//!
//! Most call sites should provide a miri compatible solution feature gated for testing.

use crate::nvim_types::{borrowed::Borrowed, Arena, Array, Dict, Object, OwnedThinString, ThinString};

// these may seem usable in normal clone implementations but they use mutable statics and non
// thread safe recovery mechanisms. Instead we do the recovery calls after ensuring they are safe.
// we call our own x*alloc definitions in nvalloc instead of these.
unsafe extern "C" {
    // NOTE: these functions return a null pointer for strings if the string points to null
    pub(crate) fn copy_dict<'a>(dict: Borrowed<'a, Dict>, arena: *mut Arena) -> Dict;
    pub(crate) fn copy_array<'a>(arr: Borrowed<'a, Array>, arena: *mut Arena) -> Array;
    pub(crate) fn copy_object<'a>(obj: Borrowed<'a, Object>, arena: *mut Arena) -> Object;
    pub(crate) fn copy_string<'a>(th: ThinString<'a>, arena: *mut Arena) -> OwnedThinString;
}
