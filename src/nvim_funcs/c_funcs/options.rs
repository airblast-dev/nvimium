use std::mem::MaybeUninit;

use crate::nvim_types::{Arena, Dict, Error, ThinString};

unsafe extern "C" {
    pub fn nvim_get_all_options_info(arena: *mut Arena, err: *mut Error) -> MaybeUninit<Dict>;
    pub fn nvim_get_option_info2<'a>(name: ThinString<'a>, arena: *mut Arena, err: *mut Error) -> MaybeUninit<Dict>;
}
