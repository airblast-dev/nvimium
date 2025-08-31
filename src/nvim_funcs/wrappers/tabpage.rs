use thread_lock::call_check;

use crate::{
    macros::tri::{tri_ez, tri_nc, tri_ret},
    nvim_funcs::c_funcs::tabpage::{
        nvim_tabpage_del_var, nvim_tabpage_get_number, nvim_tabpage_get_var, nvim_tabpage_get_win,
        nvim_tabpage_is_valid, nvim_tabpage_list_wins, nvim_tabpage_set_var, nvim_tabpage_set_win,
    },
    nvim_types::{
        AsThinString, Boolean, Error, Integer, KVec, Object, TabPage, Window, call_with_arena,
        returns::utils::ArrayOf,
    },
};

pub fn tabpage_del_var<TH: AsThinString>(tp: TabPage, name: TH) -> Result<(), Error> {
    call_check();

    tri_ez! {
        err;
        unsafe { nvim_tabpage_del_var(tp, name.as_thinstr(), &raw mut err) };
    }
}

pub fn tabpage_get_number(tp: TabPage) -> Result<Integer, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_tabpage_get_number(tp, &raw mut err)};
    }
}

pub fn tabpage_get_var<TH: AsThinString>(tp: TabPage, name: TH) -> Result<Object, Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ret! {
                err;
                nvim_tabpage_get_var(tp, name.as_thinstr(), arena, &raw mut err);
                Object::clone;
            }
        })
    }
}

pub fn tabpage_get_win<TH: AsThinString>(tp: TabPage) -> Result<Window, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_tabpage_get_win(tp, &raw mut err) };
    }
}

pub fn tabpage_is_valid(tp: TabPage) -> Boolean {
    call_check();

    unsafe { nvim_tabpage_is_valid(tp) }
}

pub fn tabpage_list_wins(tp: TabPage) -> Result<KVec<Window>, Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ret! {
                err;
                nvim_tabpage_list_wins(tp, arena, &raw mut err);
                ArrayOf::conv_to_kvec;
            }
        })
    }
}

pub fn tabpage_set_var<TH: AsThinString>(
    tp: TabPage,
    name: TH,
    value: &Object,
) -> Result<(), Error> {
    call_check();

    tri_ez! {
        err;
        unsafe { nvim_tabpage_set_var(tp, name.as_thinstr(), value.into(), &raw mut err) };
    }
}

pub fn tabpage_set_win(tp: TabPage, win: Window) -> Result<(), Error> {
    call_check();

    tri_ez! {
        err;
        unsafe { nvim_tabpage_set_win(tp, win, &raw mut err) };
    }
}
