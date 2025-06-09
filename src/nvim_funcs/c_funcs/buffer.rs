use std::mem::MaybeUninit;

use crate::nvim_types::{
    Boolean, Buffer, Channel, Error, Integer, LuaRef, Object, ThinString,
    opts::{buf_attach::BufAttachOpts, buf_delete::BufDeleteOpts},
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
}
