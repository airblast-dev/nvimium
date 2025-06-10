use std::error::Error;

use crate::{
    macros::decl_derive::derive,
    nvim_types::{
        Boolean, LuaRef,
        args::buf_attach_cb::{
            BufOnBytesArgs, BufOnChangedTickArgs, BufOnDetach, BufOnLinesArgs, BufOnReload,
        },
        lua::{Function, NvFn},
    },
};

derive!(
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    pub struct BufAttachOpts {
        #[builder_fn_skip]
        lines: LuaRef,
        #[builder_fn_skip]
        bytes: LuaRef,
        #[builder_fn_skip]
        changedtick: LuaRef,
        #[builder_fn_skip]
        detach: LuaRef,
        #[builder_fn_skip]
        reload: LuaRef,
        utf_sizes: Boolean,
        preview: Boolean,
    }
);

impl BufAttachOpts {
    pub fn lines<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnLinesArgs<'a>) -> Result<Boolean, E>,
    >(
        &mut self,
        lines: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[0];
        if self.mask & MASK == MASK {
            unsafe { self.lines.assume_init_drop() };
        }
        self.lines.write(Function::wrap(lines).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn bytes<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnBytesArgs<'a>) -> Result<Boolean, E>,
    >(
        &mut self,
        bytes: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[1];
        if self.mask & MASK == MASK {
            unsafe { self.bytes.assume_init_drop() };
        }
        self.bytes.write(Function::wrap(bytes).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn changedtick<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnChangedTickArgs<'a>) -> Result<Boolean, E> + Unpin,
    >(
        &mut self,
        changedtick: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[2];
        if self.mask & MASK == MASK {
            unsafe { self.changedtick.assume_init_drop() };
        }
        self.changedtick
            .write(Function::wrap(changedtick).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn detach<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnDetach<'a>) -> Result<(), E> + Unpin,
    >(
        &mut self,
        detach: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[3];
        if self.mask & MASK == MASK {
            unsafe { self.detach.assume_init_drop() };
        }
        self.detach.write(Function::wrap(detach).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn reload<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnReload<'a>) -> Result<(), E> + Unpin,
    >(
        &mut self,
        reload: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[4];
        if self.mask & MASK == MASK {
            unsafe { self.reload.assume_init_drop() };
        }
        self.reload.write(Function::wrap(reload).into_luaref());
        self.mask |= MASK;

        self
    }
}
