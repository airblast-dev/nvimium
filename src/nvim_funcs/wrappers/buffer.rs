use thread_lock::call_check;

use crate::{
    macros::tri::{tri_ez, tri_nc, tri_ret},
    nvim_funcs::c_funcs::buffer::{
        nvim_buf_attach, nvim_buf_call, nvim_buf_del_mark, nvim_buf_del_var, nvim_buf_delete,
        nvim_buf_get_changedtick, nvim_buf_get_keymap, nvim_buf_get_lines, nvim_buf_get_mark,
        nvim_buf_get_name, nvim_buf_get_offset, nvim_buf_get_text, nvim_buf_get_var,
    },
    nvim_types::{
        Array, AsThinString, Boolean, Buffer, CALLBACK_ARENA, Channel, Error, Integer, Object,
        OwnedThinString, ThinString,
        func_types::keymap_mode::KeyMapMode,
        lua::{Function, NvFn},
        opts::{buf_attach::BufAttachOpts, buf_delete::BufDeleteOpts, get_text::GetTextOpts},
        returns::get_keymap::Keymaps,
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

pub fn buf_get_keymap(buf: Buffer, mode: KeyMapMode) -> Result<Keymaps, Error> {
    call_check();

    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_buf_get_keymap(buf, mode, arena, &raw mut err) };
            Keymaps::from_c_func_ret;
        };

        arena.reset_pos();

        ret
    })
}

/// Get's lines of a buffer and feeds it so the provided function
///
/// The `consumer` is given an iterator of [`ThinString`]'s where their lifetime cannot leave
/// `consumer`. This is done to avoid possibly huge allocations by using existing space in the
/// arena that is already acquired.
// TODO: return dyn until an exact iterator type is decided
pub fn buf_get_lines<R, F: for<'a> FnMut(&'a mut dyn Iterator<Item = ThinString<'a>>) -> R>(
    mut consumer: F,
    buf: Buffer,
    start: Integer,
    end: Integer,
    strict_indexing: Boolean,
) -> Result<R, Error> {
    call_check();

    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_buf_get_lines(Channel::LUA_INTERNAL_CALL, buf, start, end, strict_indexing, arena, core::ptr::null_mut(), &raw mut err) };
            (|arr: &Array| {
                let mut iter = arr.iter().map(|obj| obj.as_string().unwrap().as_thinstr());
                (consumer)(&mut iter)
            });
        };

        arena.reset_pos();

        ret
    })
}

pub fn buf_get_mark<TH: AsThinString>(buf: Buffer, name: TH) -> Result<(Integer, Integer), Error> {
    call_check();

    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_buf_get_mark(buf, name.as_thinstr(), arena, &raw mut err) };
            (|arr: &Array| {
                let pos = arr.as_slice();
                (pos[0].as_int().unwrap(), pos[1].as_int().unwrap())
            });
        };

        arena.reset_pos();
        ret
    })
}

pub fn buf_get_name(buf: Buffer) -> Result<OwnedThinString, Error> {
    call_check();

    tri_ret! {
        err;
        unsafe { nvim_buf_get_name(buf, &raw mut err) };
        (|s: &OwnedThinString| s.clone());
    }
}

pub fn buf_get_offset(buf: Buffer, index: Integer) -> Result<Integer, Error> {
    call_check();

    tri_nc! {
        err;
        unsafe { nvim_buf_get_offset(buf, index, &raw mut err) };
    }
}

/// Get's partial lines of a buffer and feeds it so the provided function
///
/// The `consumer` is given an iterator of [`ThinString`]'s where their lifetime cannot leave
/// `consumer`. This is done to avoid possibly huge allocations by using existing space in the
/// arena that is already acquired.
// TODO: return dyn until an exact iterator type is decided
pub fn buf_get_text<R, F: for<'a> FnMut(&'a mut dyn Iterator<Item = ThinString<'a>>) -> R>(
    mut consumer: F,
    buf: Buffer,
    start_row: Integer,
    start_col: Integer,
    end_row: Integer,
    end_col: Integer,
    opts: &mut GetTextOpts,
) -> Result<R, Error> {
    call_check();

    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_buf_get_text(Channel::LUA_INTERNAL_CALL, buf, start_row, start_col, end_row, end_col, opts, arena, core::ptr::null_mut(), &raw mut err) };
            (|arr: &Array| {
                let mut iter = arr.iter().map(|obj| obj.as_string().unwrap().as_thinstr());
                (consumer)(&mut iter)
            });
        };

        arena.reset_pos();

        ret
    })
}

pub fn buf_get_var<TH: AsThinString>(buf: Buffer, name: TH) -> Result<Object, Error> {
    call_check();

    CALLBACK_ARENA.with_borrow_mut(|arena| {
        let ret = tri_ret! {
            err;
            unsafe { nvim_buf_get_var(buf, name.as_thinstr(), arena, &raw mut err) };
            Object::clone;
        };

        arena.reset_pos();
        ret
    })
}
