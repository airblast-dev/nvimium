use std::mem::MaybeUninit;

use crate::nvim_types::{
    Arena, Boolean, Error, Integer, Object, TabPage, ThinString, Window, borrowed::Borrowed,
    returns::utils::ArrayOf,
};

unsafe extern "C" {
    pub fn nvim_tabpage_del_var(tp: TabPage, name: ThinString<'_>, err: *mut Error);
    pub fn nvim_tabpage_get_number(tp: TabPage, error: *mut Error) -> MaybeUninit<Integer>;
    pub fn nvim_tabpage_get_var(
        tp: TabPage,
        name: ThinString<'_>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_tabpage_get_win(tp: TabPage, err: *mut Error) -> MaybeUninit<Window>;
    pub fn nvim_tabpage_is_valid(tp: TabPage) -> Boolean;
    pub fn nvim_tabpage_list_wins(
        tp: TabPage,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<ArrayOf<Window>>;
    pub fn nvim_tabpage_set_var(
        tp: TabPage,
        name: ThinString<'_>,
        value: Borrowed<'_, Object>,
        err: *mut Error,
    );
    pub fn nvim_tabpage_set_win(tp: TabPage, win: Window, err: *mut Error);
}
