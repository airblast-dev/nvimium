use thread_lock::call_check;

use crate::{
    macros::tri::tri_ret,
    nvim_funcs::c_funcs::options::{nvim_get_all_options_info, nvim_get_option_info2},
    nvim_types::{
        Arena, AsThinString, Error,
        returns::options_info::{OptionInfo, OptionsInfo},
    },
};

pub fn get_all_options() -> Result<OptionsInfo, Error> {
    call_check();
    let mut arena = Arena::EMPTY;
    tri_ret! {
        err;
        unsafe { nvim_get_all_options_info(&raw mut arena, &raw mut err) };
        OptionsInfo::from_c_func_ret;
    }
}

pub fn get_options_info2<TH: AsThinString>(name: TH) -> Result<OptionInfo, Error> {
    call_check();
    let mut arena = Arena::EMPTY;
    tri_ret! {
        err;
        unsafe { nvim_get_option_info2(name.as_thinstr(), &raw mut arena, &raw mut err) };
        OptionInfo::from_c_func_ret;
    }
}
