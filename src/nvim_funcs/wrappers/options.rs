use thread_lock::call_check;

use crate::{
    macros::tri::{tri_nc, tri_ret},
    nvim_funcs::c_funcs::options::{
        nvim_get_all_options_info, nvim_get_option_info2, nvim_get_option_value,
    },
    nvim_types::{
        AsThinString, CALLBACK_ARENA, Error, Object,
        opts::option::OptionOpt,
        returns::options_info::{OptionInfo, OptionsInfo},
    },
};

pub fn get_all_options() -> Result<OptionsInfo, Error> {
    call_check();

    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_get_all_options_info(arena, &raw mut err) };
            OptionsInfo::from_c_func_ret;
        };

        arena.reset_pos();
        ret
    })
}

pub fn get_options_info2<TH: AsThinString>(name: TH) -> Result<OptionInfo, Error> {
    call_check();

    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_get_option_info2(name.as_thinstr(), arena, &raw mut err) };
            OptionInfo::from_c_func_ret;
        };

        arena.reset_pos();
        ret
    })
}

pub fn get_option_value<'a, TH: AsThinString>(name: TH, opt: &mut OptionOpt<'a>) -> Result<Object, Error> {
    call_check();

    tri_nc! {
        err; 
        unsafe { nvim_get_option_value(name.as_thinstr(), opt, &raw mut err) }; 
    }
}
