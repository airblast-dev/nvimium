use std::mem::MaybeUninit;

use crate::nvim_types::{
    Arena, Channel, Dict, Error, Object, ThinString, borrowed::Borrowed, opts::option::OptionOpt,
};

unsafe extern "C" {
    pub fn nvim_get_all_options_info(arena: *mut Arena, err: *mut Error) -> MaybeUninit<Dict>;
    pub fn nvim_get_option_info2(
        name: ThinString<'_>,
        opt: *mut OptionOpt,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Dict>;
    pub fn nvim_get_option_value(
        name: ThinString<'_>,
        opt: *mut OptionOpt,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_set_option_value(
        chan: Channel,
        name: ThinString<'_>,
        value: Borrowed<'_, Object>,
        opts: *mut OptionOpt,
        err: *mut Error,
    );
}
