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

#[cfg(all(not(miri), feature = "testing"))]
mod tests {
    use mlua_sys::LUA_NOREF;

    use crate::{
        self as nvimium,
        nvim_funcs::tabpage::{tabpage_get_var, tabpage_set_var},
        nvim_types::{Error, Object, OwnedThinString, TabPage, lua::Function},
    };

    #[nvim_test::nvim_test]
    fn set_get_tabpage_value() {
        tabpage_get_var(TabPage::new(0), c"super_secret_value").unwrap_err();

        tabpage_set_var(
            TabPage::new(0),
            c"super_secret_value",
            &Object::String(OwnedThinString::from(c"important string")),
        )
        .unwrap();

        let val = tabpage_get_var(TabPage::new(0), c"super_secret_value").unwrap();
        assert_eq!(
            val,
            Object::String(OwnedThinString::from(c"important string"))
        );

        let cb = Function::wrap(|_: ()| Ok::<(), Error>(())).into_luaref();
        let ref_obj = Object::LuaRef(cb);
        tabpage_set_var(
            TabPage::new(0),
            c"super_secret_value",
            &ref_obj,
        )
        .unwrap();

        // esnure that we dont double free the lua reference as this function should not
        // take/mutate the ref but instead should take a new reference to it
        assert_ne!(ref_obj.into_luaref().unwrap().0, LUA_NOREF);

        let val = tabpage_get_var(TabPage::new(0), c"super_secret_value").unwrap();
        assert!(val.into_luaref().is_some_and(|v| v.0 != LUA_NOREF));

    }
}
