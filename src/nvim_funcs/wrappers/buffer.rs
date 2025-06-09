use thread_lock::call_check;

use crate::{
    macros::tri::{tri_ez, tri_nc},
    nvim_funcs::c_funcs::buffer::{
        nvim_buf_attach, nvim_buf_call, nvim_buf_del_mark, nvim_buf_del_var, nvim_buf_delete,
        nvim_buf_get_changedtick,
    },
    nvim_types::{
        AsThinString, Boolean, Buffer, Channel, Error, Integer, Object,
        lua::{Function, NvFn},
        opts::{buf_attach::BufAttachOpts, buf_delete::BufDeleteOpts},
    },
    plugin::IntoLua,
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

pub fn buf_call<
    Err: 'static + std::error::Error,
    Ret: 'static + IntoLua,
    F: NvFn + Fn(()) -> Result<Ret, Err>,
>(
    buf: Buffer,
    f: F,
) -> Result<Object, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_buf_call(buf, Function::wrap(f).into_luaref(), &raw mut err) };
    }
}

pub fn buf_del_mark<TH: AsThinString>(buf: Buffer, name: TH) -> Result<Boolean, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_buf_del_mark(buf, name.as_thinstr(), &raw mut err) };
    }
}

pub fn buf_del_var<TH: AsThinString>(buf: Buffer, name: TH) -> Result<(), Error> {
    call_check();

    tri_ez! {
        err;
        unsafe { nvim_buf_del_var(buf, name.as_thinstr(), &raw mut err) };
    }
}

pub fn buf_delete(buf: Buffer, opts: &mut BufDeleteOpts) -> Result<(), Error> {
    call_check();

    tri_ez! {
        err;
        unsafe { nvim_buf_delete(buf, opts, &raw mut err) };
    }
}

pub fn buf_get_changedtick(buf: Buffer) -> Result<Integer, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_buf_get_changedtick(buf, &raw mut err) };
    }
}
