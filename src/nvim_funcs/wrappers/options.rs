use thread_lock::call_check;

use crate::{
    macros::tri::{tri_ez, tri_nc, tri_ret},
    nvim_funcs::c_funcs::options::{
        nvim_get_all_options_info, nvim_get_option_info2, nvim_get_option_value,
        nvim_set_option_value,
    },
    nvim_types::{
        AsThinString, CALLBACK_ARENA, Channel, Error, Object,
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

pub fn get_option_value<'a, TH: AsThinString>(
    name: TH,
    opts: &mut OptionOpt<'a>,
) -> Result<Object, Error> {
    call_check();

    tri_nc! {
        err;
        // returns fully allocated object
        unsafe { nvim_get_option_value(name.as_thinstr(), opts, &raw mut err) };
    }
}

pub fn set_option_value<'a, TH: AsThinString>(
    name: TH,
    value: Object,
    opts: &mut OptionOpt<'a>,
) -> Result<(), Error> {
    call_check();

    tri_ez! {
        err;
        unsafe { nvim_set_option_value(Channel::LUA_INTERNAL_CALL, name.as_thinstr(), (&value).into(), opts, &raw mut err) };
    }
}
