use thread_lock::call_check;

use crate::{
    macros::tri::{tri_ez, tri_nc, tri_ret},
    nvim_funcs::c_funcs::buffer::{
        nvim_buf_attach, nvim_buf_call, nvim_buf_del_mark, nvim_buf_del_var, nvim_buf_delete,
        nvim_buf_get_changedtick, nvim_buf_get_keymap, nvim_buf_get_lines, nvim_buf_get_mark,
        nvim_buf_get_name, nvim_buf_get_offset, nvim_buf_get_text, nvim_buf_get_var,
        nvim_buf_is_loaded, nvim_buf_is_valid, nvim_buf_line_count, nvim_buf_set_keymap,
        nvim_buf_set_lines, nvim_buf_set_mark, nvim_buf_set_name, nvim_buf_set_text,
        nvim_buf_set_var,
    },
    nvim_types::{
        Array, AsThinString, Boolean, Buffer, Channel, Error, Integer, IntoLua, Object,
        OwnedThinString, call_with_arena,
        func_types::keymap_mode::KeyMapMode,
        iter::ThIter,
        lua::{Function, NvFn},
        opts::{
            buf_attach::BufAttachOpts, buf_delete::BufDeleteOpts, get_text::GetTextOpts,
            set_keymap::SetKeymapOpts, set_mark::SetMarkOpts,
        },
        returns::get_keymap::Keymaps,
    },
};

pub fn buf_attach(
    buf: Buffer,
    send_buffer: Boolean,
    opts: &mut BufAttachOpts,
) -> Result<Boolean, Error> {
    call_check();

    unsafe {
        tri_nc! {
            err;
            nvim_buf_attach(Channel::LUA_INTERNAL_CALL, buf, send_buffer, opts, &raw mut err);
        }
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

    unsafe {
        tri_nc! {
            err;
            nvim_buf_call(buf, Function::wrap(f).into_luaref(), &raw mut err);
        }
    }
}

pub fn buf_del_mark<TH: AsThinString>(buf: Buffer, name: TH) -> Result<Boolean, Error> {
    call_check();

    unsafe {
        tri_nc! {
            err;
            nvim_buf_del_mark(buf, name.as_thinstr(), &raw mut err);
        }
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

    unsafe {
        tri_nc! {
            err;
            nvim_buf_get_changedtick(buf, &raw mut err);
        }
    }
}

pub fn buf_get_keymap(buf: Buffer, mode: KeyMapMode) -> Result<Keymaps, Error> {
    call_check();

    unsafe {
        call_with_arena(move |arena| {
            tri_ret! {
                err;
                nvim_buf_get_keymap(buf, mode, arena, &raw mut err);
                Keymaps::from_c_func_ret;
            }
        })
    }
}

/// Get's lines of a buffer and feeds it so the provided function
///
/// The `consumer` is given an iterator of [`ThinString`]'s where their lifetime cannot leave
/// `consumer`. This is done to avoid possibly huge allocations by using existing space in the
/// arena that is already acquired.
// TODO: return dyn until an exact iterator type is decided
pub fn buf_get_lines<R, F: for<'a> FnMut(ThIter<'a>) -> R>(
    mut consumer: F,
    buf: Buffer,
    start: Integer,
    end: Integer,
    strict_indexing: Boolean,
) -> Result<R, Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ret! {
                err;
                nvim_buf_get_lines(Channel::LUA_INTERNAL_CALL, buf, start, end, strict_indexing, arena, core::ptr::null_mut(), &raw mut err);
                (|arr: &Array| {
                    (consumer)(ThIter::new(arr.as_slice()))
                });
            }
        })
    }
}

pub fn buf_get_mark<TH: AsThinString>(buf: Buffer, name: TH) -> Result<(Integer, Integer), Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ret! {
                err;
                nvim_buf_get_mark(buf, name.as_thinstr(), arena, &raw mut err);
                (|arr: &Array| {
                    let pos = arr.as_slice();
                    (pos[0].as_int().unwrap(), pos[1].as_int().unwrap())
                });
            }
        })
    }
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

    unsafe {
        tri_nc! {
            err;
            nvim_buf_get_offset(buf, index, &raw mut err);
        }
    }
}

/// Get's partial lines of a buffer and feeds it so the provided function
///
/// The `consumer` is given an iterator of [`ThinString`]'s where their lifetime cannot leave
/// `consumer`. This is done to avoid possibly huge allocations by using existing space in the
/// arena that is already acquired.
// TODO: return dyn until an exact iterator type is decided
pub fn buf_get_text<R, F: for<'a> FnMut(ThIter<'a>) -> R>(
    mut consumer: F,
    buf: Buffer,
    start_row: Integer,
    start_col: Integer,
    end_row: Integer,
    end_col: Integer,
    opts: &mut GetTextOpts,
) -> Result<R, Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ret! {
                err;
                nvim_buf_get_text(Channel::LUA_INTERNAL_CALL, buf, start_row, start_col, end_row, end_col, opts, arena, core::ptr::null_mut(), &raw mut err);
                (|arr: &Array| (consumer)(ThIter::new(arr.as_slice())));
            }
        })
    }
}

pub fn buf_get_var<TH: AsThinString>(buf: Buffer, name: TH) -> Result<Object, Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ret! {
                err;
                nvim_buf_get_var(buf, name.as_thinstr(), arena, &raw mut err);
                Object::clone;
            }
        })
    }
}

pub fn buf_is_loaded(buf: Buffer) -> Boolean {
    call_check();

    unsafe { nvim_buf_is_loaded(buf) }
}

pub fn buf_is_valid(buf: Buffer) -> Boolean {
    call_check();

    unsafe { nvim_buf_is_valid(buf) }
}

pub fn buf_line_count(buf: Buffer) -> Result<Integer, Error> {
    call_check();

    unsafe {
        tri_nc! {
            err;
            nvim_buf_line_count(buf, &raw mut err);
        }
    }
}

pub fn buf_set_keymap<TH: AsThinString, TH2: AsThinString>(
    buf: Buffer,
    mode: KeyMapMode,
    lhs: TH,
    rhs: TH2,
    opts: &mut SetKeymapOpts,
) -> Result<(), Error> {
    call_check();

    unsafe {
        tri_ez! {
            err;
            nvim_buf_set_keymap(Channel::LUA_INTERNAL_CALL, buf, mode, lhs.as_thinstr(), rhs.as_thinstr(), opts, &raw mut err);
        }
    }
}

pub fn buf_set_lines(
    buf: Buffer,
    start: Integer,
    end: Integer,
    strict_indexing: Boolean,
    replacement: &Array,
) -> Result<(), Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ez! {
                err;
                nvim_buf_set_lines(Channel::LUA_INTERNAL_CALL, buf, start, end, strict_indexing, replacement.into(), arena, &raw mut err);
            }
        })
    }
}

pub fn buf_set_mark<TH: AsThinString>(
    buf: Buffer,
    name: TH,
    line: Integer,
    col: Integer,
    opts: &mut SetMarkOpts,
) -> Result<Boolean, Error> {
    call_check();

    unsafe {
        tri_nc! {
            err;
            nvim_buf_set_mark(buf, name.as_thinstr(), line, col, opts, &raw mut err);
        }
    }
}

pub fn buf_set_name<TH: AsThinString>(buf: Buffer, name: TH) -> Result<(), Error> {
    call_check();

    unsafe {
        tri_ez! {
            err;
            nvim_buf_set_name(buf, name.as_thinstr(), &raw mut err);
        }
    }
}

pub fn buf_set_text(
    buf: Buffer,
    start_row: Integer,
    start_col: Integer,
    end_row: Integer,
    end_col: Integer,
    replacement: &Array,
) -> Result<(), Error> {
    call_check();

    unsafe {
        call_with_arena(|arena| {
            tri_ez! {
                err;
                nvim_buf_set_text(Channel::LUA_INTERNAL_CALL, buf, start_row, start_col, end_row, end_col, replacement.into(), arena, &raw mut err);
            }
        })
    }
}

pub fn buf_set_var<TH: AsThinString>(buf: Buffer, name: TH, val: &Object) -> Result<(), Error> {
    call_check();

    unsafe {
        tri_ez! {
            err;
            nvim_buf_set_var(buf, name.as_thinstr(), val.into(), &raw mut err);
        }
    }
}

#[cfg(all(not(miri), feature = "testing"))]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::{buf_get_var, buf_set_var};
    use crate::{
        self as nvimium,
        nvim_funcs::global::paste,
        nvim_types::{
            Boolean, Buffer, Error, Object,
            opts::{buf_attach::BufAttachOpts, paste::PastePhase},
        },
        th,
    };

    #[nvim_test::nvim_test]
    fn buf_attach() {
        // flags that we set when our callback is invoked
        static ON_BYTES_FLAG: AtomicBool = AtomicBool::new(false);
        static ON_LINES_FLAG: AtomicBool = AtomicBool::new(false);
        super::buf_attach(
            Buffer::new(0),
            true,
            BufAttachOpts::default()
                .on_bytes(move |args| {
                    assert_eq!(args.source, th!("bytes"));
                    ON_BYTES_FLAG.store(true, Ordering::SeqCst);
                    Ok::<Boolean, Error>(true)
                })
                .on_lines(move |args| {
                    assert_eq!(args.source, th!("lines"));
                    assert!(args.deleted_codeunits.is_some());
                    assert!(args.deleted_codepoints.is_some());
                    ON_LINES_FLAG.store(true, Ordering::SeqCst);
                    Ok::<Boolean, Error>(true)
                })
                .utf_sizes(true),
        )
        .unwrap();

        paste(c"SomeText", false, PastePhase::Single).unwrap();

        assert!(ON_BYTES_FLAG.load(Ordering::SeqCst));
        assert!(ON_LINES_FLAG.load(Ordering::SeqCst));
    }

    // LATER

    #[nvim_test::nvim_test]
    fn get_set_var() {
        buf_set_var(Buffer::new(0), c"NvimiumEpicVar", &Object::Bool(true)).unwrap();
        let var = buf_get_var(Buffer::new(0), c"NvimiumEpicVar").unwrap();
        assert_eq!(var, Object::Bool(true));
    }
}
