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

#[cfg(all(not(miri), feature = "testing"))]
mod tests {
    use crate as nvimium;
    use crate::{
        nvim_funcs::options::{get_all_options, set_option_value},
        nvim_types::{
            Dict, Object,
            opts::option::{OptionOpt, OptionScope},
        },
    };

    use super::get_option_value;

    #[nvim_test::nvim_test]
    fn set_get_all_options() {
        let options = get_all_options().unwrap();
        assert!(!options.options.into_iter().any(|opt| opt.name == c"rule"));
    }

    #[nvim_test::nvim_test]
    fn set_get_option_value() {
        let val =
            get_option_value(c"ruler", OptionOpt::default().scope(OptionScope::Global)).unwrap();
        assert_eq!(val, Object::Bool(true));

        set_option_value(
            c"ruler",
            Object::Bool(false),
            OptionOpt::default().scope(OptionScope::Global),
        )
        .unwrap();

        let val =
            get_option_value(c"ruler", OptionOpt::default().scope(OptionScope::Global)).unwrap();
        assert_eq!(val, Object::Bool(false));

        set_option_value(
            c"ruf",
            Object::String(c"%15(%c%V\\ %p%%%)".into()),
            OptionOpt::default().scope(OptionScope::Global),
        )
        .unwrap();

        let val =
            get_option_value(c"ruf", OptionOpt::default().scope(OptionScope::Global)).unwrap();

        assert_eq!(val, Object::String(c"%15(%c%V\\ %p%%%)".into()));

        set_option_value(
            c"ruf",
            Object::Dict(Dict::default()),
            OptionOpt::default().scope(OptionScope::Global),
        )
        .unwrap_err();

        set_option_value(
            c"NvimiumFakeOption",
            Object::Dict(Dict::default()),
            OptionOpt::default().scope(OptionScope::Global),
        )
        .unwrap_err();
    }
}
