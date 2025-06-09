use thread_lock::call_check;

use crate::{
    macros::tri::tri_nc,
    nvim_funcs::c_funcs::buffer::nvim_buf_attach,
    nvim_types::{Boolean, Buffer, Channel, Error, opts::buf_attach::BufAttachOpts},
};

pub fn buf_attach(
    buf: Buffer,
    send_buffer: Boolean,
    opts: &mut BufAttachOpts,
) -> Result<Boolean, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_buf_attach(Channel::LUA_INTERNAL_CALL, buf, send_buffer, opts, &raw mut err) };
    }
}
