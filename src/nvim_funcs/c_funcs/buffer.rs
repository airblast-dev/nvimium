use std::mem::MaybeUninit;

use mlua_sys::lua_State;

use crate::nvim_types::{
    Arena, Array, Boolean, Buffer, Channel, Error, Integer, LuaRef, Object, OwnedThinString,
    ThinString,
    borrowed::Borrowed,
    func_types::keymap_mode::KeyMapMode,
    opts::{
        buf_attach::BufAttachOpts, buf_delete::BufDeleteOpts, get_text::GetTextOpts,
        set_keymap::SetKeymapOpts, set_mark::SetMarkOpts,
    },
};

unsafe extern "C" {
    pub fn nvim_buf_attach(
        chan: Channel,
        buf: Buffer,
        send_buffer: Boolean,
        opts: *mut BufAttachOpts,
        err: *mut Error,
    ) -> MaybeUninit<Boolean>;

    pub fn nvim_buf_call(buf: Buffer, f: LuaRef, err: *mut Error) -> MaybeUninit<Object>;
    pub fn nvim_buf_del_mark<'a>(
        buf: Buffer,
        name: ThinString<'a>,
        err: *mut Error,
    ) -> MaybeUninit<Boolean>;

    pub fn nvim_buf_del_var<'a>(buf: Buffer, name: ThinString<'a>, err: *mut Error);
    pub fn nvim_buf_delete(buf: Buffer, opts: *mut BufDeleteOpts, err: *mut Error);
    pub fn nvim_buf_get_changedtick(buf: Buffer, err: *mut Error) -> MaybeUninit<Integer>;
    pub fn nvim_buf_get_keymap(
        buf: Buffer,
        mode: KeyMapMode,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_buf_get_lines(
        chan: Channel,
        buf: Buffer,
        start: Integer,
        end: Integer,
        strict_indexing: Boolean,
        arena: *mut Arena,
        l: *mut lua_State,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_buf_get_mark<'a>(
        buf: Buffer,
        name: ThinString<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_buf_get_name(buf: Buffer, err: *mut Error) -> MaybeUninit<OwnedThinString>;
    pub fn nvim_buf_get_offset(
        buf: Buffer,
        index: Integer,
        err: *mut Error,
    ) -> MaybeUninit<Integer>;
    pub fn nvim_buf_get_text(
        chan: Channel,
        buf: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: *mut GetTextOpts,
        arena: *mut Arena,
        l: *mut lua_State,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
    pub fn nvim_buf_get_var<'a>(
        buf: Buffer,
        name: ThinString<'a>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
    pub fn nvim_buf_is_loaded(buf: Buffer) -> Boolean;
    pub fn nvim_buf_is_valid(buf: Buffer) -> Boolean;
    pub fn nvim_buf_line_count(buf: Buffer, err: *mut Error) -> MaybeUninit<Integer>;
    pub fn nvim_buf_set_keymap<'a>(
        chan: Channel,
        buf: Buffer,
        mode: KeyMapMode,
        lhs: ThinString<'a>,
        rhs: ThinString<'a>,
        opts: *mut SetKeymapOpts,
        err: *mut Error,
    );
    pub fn nvim_buf_set_lines<'a>(
        chan: Channel,
        buf: Buffer,
        start: Integer,
        end: Integer,
        strict_indexing: Boolean,
        replacement: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    );

    pub fn nvim_buf_set_mark<'a>(
        buf: Buffer,
        name: ThinString<'a>,
        line: Integer,
        col: Integer,
        opts: *mut SetMarkOpts,
        err: *mut Error,
    ) -> MaybeUninit<Boolean>;
    pub fn nvim_buf_set_name<'a>(buf: Buffer, name: ThinString<'a>, err: *mut Error);
    pub fn nvim_buf_set_text<'a>(
        chan: Channel,
        buf: Buffer,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        replacement: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    );
    pub fn nvim_buf_set_var<'a>(
        buf: Buffer,
        name: ThinString<'a>,
        val: Borrowed<'a, Object>,
        err: *mut Error,
    );
}
