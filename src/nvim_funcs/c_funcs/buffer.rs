use std::mem::MaybeUninit;

use crate::nvim_types::{lua::Function, opts::buf_attach::BufAttachOpts, Boolean, Buffer, Channel, Error};

unsafe extern "C" {
    pub fn nvim_buf_attach(
        chan: Channel,
        buf: Buffer,
        send_buffer: Boolean,
        opts: *mut BufAttachOpts,
        err: *mut Error,
    ) -> MaybeUninit<Boolean>;

    //pub fn nvim_buf_call(buf: Buffer, f: Function)
}
