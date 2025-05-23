use std::{mem::discriminant, ops::Deref};

use libc::c_int;
use mlua_sys::LUA_NOREF;

use crate::nvim_types::{
    Array, Boolean, Buffer, Dict, Integer, KVec, LuaRef, Object, OwnedThinString,
    object_subs::LuaRefOrString,
};

use super::utils::skip_drop_remove_keys;

pub struct Keymaps {
    pub maps: KVec<Keymap>,
}

impl Keymaps {
    pub(crate) fn from_c_func_ret(arr: &mut Array) -> Self {
        let mut kv = KVec::with_capacity(arr.len());
        kv.extend(arr.iter_mut().filter_map(|obj| match obj {
            Object::Dict(d) => Some(Keymap::from_c_func_ret(d)),
            _ => None,
        }));

        Self { maps: kv }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Keymap {
    pub rhs: LuaRefOrString,
    pub desc: Option<OwnedThinString>,
    pub lhs: OwnedThinString,
    pub lhsraw: OwnedThinString,
    pub lhsrawalt: Option<OwnedThinString>,
    pub noremap: Integer,
    pub script: Boolean,
    pub expr: Boolean,
    pub silent: Boolean,
    pub sid: Integer,
    pub scriptversion: Integer,
    pub lnum: Integer,
    pub buffer: Buffer,
    pub nowait: Boolean,
    pub replace_keycodes: Boolean,
    pub mode: OwnedThinString,
    pub abbr: Boolean,
    pub mode_bits: Integer,
}

impl Keymap {
    #[inline]
    pub(crate) fn from_c_func_ret(d: &mut Dict) -> Self {
        let [
            callback,
            rhs,
            desc,
            lhs,
            lhsraw,
            lhsrawalt,
            noremap,
            script,
            expr,
            silent,
            sid,
            scriptversion,
            lnum,
            buffer,
            nowait,
            replace_keycodes,
            mode,
            abbr,
            mode_bits,
        ] = skip_drop_remove_keys(
            d,
            &[
                "callback",
                "rhs",
                "desc",
                "lhs",
                "lhsraw",
                "lhsrawalt",
                "noremap",
                "script",
                "expr",
                "silent",
                "sid",
                "scriptversion",
                "lnum",
                "buffer",
                "nowait",
                "replace_keycodes",
                "mode",
                "abbr",
                "mode_bits",
            ],
            Some(|key| match key {
                "callback" | "desc" | "lhsrawalt" | "rhs" => Some(Object::Null),
                // this is either present with true, or missing which means false
                "replace_keycodes" => Some(Object::Bool(false)),
                _ => None,
            }),
        )
        .unwrap();

        let callback = match callback.deref() {
            Object::LuaRef(lref) if lref.0 != LUA_NOREF => unsafe {
                // we need a copy of the value as `callback`
                // its not dropped as its wrapped with ManuallyDrop so no double unref
                Some((lref as *const LuaRef).read())
            },
            _ => None,
        };
        let rhs = match rhs.deref() {
            Object::String(s) => Some(s.clone()),
            _ => None,
        };

        let rhs = match (rhs, callback) {
            // even if both are present callback takes precedence
            (_, Some(callback)) => LuaRefOrString::from(callback),
            (Some(s), _) => LuaRefOrString::from(s),
            (None, None) => unreachable!("Nvimium: got keymap without rhs or callback"),
        };

        let desc = match desc.deref() {
            Object::String(s) => Some(s.clone()),
            _ => None,
        };

        let lhs = match lhs.deref() {
            Object::String(s) => s.clone(),
            obj => unreachable!(
                "Nvimium: got keymap with non string lhs, kind = {:?}",
                discriminant(obj)
            ),
        };

        let lhsraw = match lhsraw.deref() {
            Object::String(s) => s.clone(),
            obj => unreachable!(
                "Nvimium: got keymap with non string lhsraw, kind = {:?}",
                discriminant(obj)
            ),
        };

        let lhsrawalt = match lhsrawalt.deref() {
            Object::String(s) => Some(s.clone()),
            Object::Null => None,
            obj => unreachable!(
                "Nvimium: got keymap with non string lhsrawalt, kind = {:?}",
                discriminant(obj)
            ),
        };

        let noremap = match noremap.deref() {
            Object::Integer(n) => *n,
            obj => unreachable!(
                "Nvimium: got keymap with non integer noremap, kind = {:?}",
                discriminant(obj)
            ),
        };

        let script = match script.deref() {
            Object::Integer(n) => *n != 0,
            Object::Bool(b) => *b,
            obj => unreachable!(
                "Nvimium: got keymap with non boolean script, kind = {:?}",
                discriminant(obj)
            ),
        };

        let expr = match expr.deref() {
            Object::Integer(n) => *n != 0,
            Object::Bool(b) => *b,
            obj => unreachable!(
                "Nvimium: got keymap with non boolean expr, kind = {:?}",
                discriminant(obj)
            ),
        };

        let silent = match silent.deref() {
            Object::Integer(n) => *n != 0,
            Object::Bool(b) => *b,
            obj => unreachable!(
                "Nvimium: got keymap with non boolean silent, kind = {:?}",
                discriminant(obj)
            ),
        };

        let sid = match sid.deref() {
            Object::Integer(n) => *n,
            obj => unreachable!(
                "Nvimium: got keymap with non boolean sid, kind = {:?}",
                discriminant(obj)
            ),
        };

        let scriptversion = match scriptversion.deref() {
            Object::Integer(n) => *n,
            // technically not correct but neovim seems to just return 1 in all code paths i was
            // able to find, just do the same as a fallback
            _ => 1,
        };

        let lnum = match lnum.deref() {
            Object::Integer(n) => *n,
            obj => unreachable!(
                "Nvimium: got keymap with non integer lnum, kind = {:?}",
                discriminant(obj)
            ),
        };

        let buffer = match buffer.deref() {
            // for some reason this is returned as an Integer instead of a buffer value
            Object::Integer(n) => Buffer::new(*n as c_int),
            Object::Buffer(b) => *b,
            obj => unreachable!(
                "Nvimium: got keymap with non integer buffer, kind = {:?}",
                discriminant(obj)
            ),
        };

        let nowait = match nowait.deref() {
            Object::Integer(n) => *n != 0,
            Object::Bool(b) => *b,
            obj => unreachable!(
                "Nvimium: got keymap with non boolean nowait, kind = {:?}",
                discriminant(obj)
            ),
        };

        let replace_keycodes = match replace_keycodes.deref() {
            Object::Integer(n) => *n != 0,
            Object::Bool(b) => *b,
            obj => unreachable!(
                "Nvimium: got keymap with non boolean replace_keycodes, kind = {:?}",
                discriminant(obj)
            ),
        };

        let mode = match mode.deref() {
            Object::String(m) => m.clone(),
            obj => unreachable!(
                "Nvimium: got keymap with non string mode, kind = {:?}",
                discriminant(obj)
            ),
        };

        let abbr = match abbr.deref() {
            Object::Integer(n) => *n != 0,
            Object::Bool(b) => *b,
            obj => unreachable!(
                "Nvimium: got keymap with non boolean abbr, kind = {:?}",
                discriminant(obj)
            ),
        };

        let mode_bits = match mode_bits.deref() {
            Object::Integer(n) => *n,
            obj => unreachable!(
                "Nvimium: got keymap with non integer mode_bits, kind = {:?}",
                discriminant(obj)
            ),
        };
        Self {
            rhs,
            desc,
            lhs,
            lhsraw,
            lhsrawalt,
            noremap,
            script,
            expr,
            silent,
            sid,
            scriptversion,
            lnum,
            buffer,
            nowait,
            replace_keycodes,
            mode,
            abbr,
            mode_bits,
        }
    }
}
