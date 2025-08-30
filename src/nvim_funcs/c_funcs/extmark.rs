use std::mem::MaybeUninit;

use crate::nvim_types::{
    Arena, Array, Boolean, Buffer, Error, Integer, NameSpace, extmark::ExtMark,
    opts::get_extmark::GetExtmarkOpts,
};

unsafe extern "C" {
    pub fn nvim_buf_clear_namespace(
        buf: Buffer,
        ns: NameSpace,
        line_start: Integer,
        line_end: Integer,
        err: *mut Error,
    );

    pub fn nvim_buf_del_extmark(
        buf: Buffer,
        ns: NameSpace,
        ex: ExtMark,
        err: *mut Error,
    ) -> MaybeUninit<Boolean>;

    pub fn nvim_buf_get_extmark_by_id(
        buf: Buffer,
        ns: NameSpace,
        ex: ExtMark,
        opts: *mut GetExtmarkOpts,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Array>;
}
