use std::mem::MaybeUninit;

use crate::nvim_types::{
    Boolean, Buffer, Channel, Error, LuaRef, Object,
    lua::{Function, NvFn},
    opts::buf_attach::BufAttachOpts,
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
}
