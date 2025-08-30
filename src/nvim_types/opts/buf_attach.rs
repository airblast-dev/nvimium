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
        on_lines: LuaRef,
        #[builder_fn_skip]
        on_bytes: LuaRef,
        #[builder_fn_skip]
        on_changedtick: LuaRef,
        #[builder_fn_skip]
        on_detach: LuaRef,
        #[builder_fn_skip]
        on_reload: LuaRef,
        utf_sizes: Boolean,
        preview: Boolean,
    }
);

impl BufAttachOpts {
    pub fn on_lines<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnLinesArgs<'a>) -> Result<Boolean, E>,
    >(
        &mut self,
        lines: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[0];
        if self.mask & MASK == MASK {
            unsafe { self.on_lines.assume_init_drop() };
        }
        self.on_lines.write(Function::wrap(lines).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn on_bytes<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnBytesArgs<'a>) -> Result<Boolean, E>,
    >(
        &mut self,
        bytes: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[1];
        if self.mask & MASK == MASK {
            unsafe { self.on_bytes.assume_init_drop() };
        }
        self.on_bytes.write(Function::wrap(bytes).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn on_changedtick<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnChangedTickArgs<'a>) -> Result<Boolean, E> + Unpin,
    >(
        &mut self,
        changedtick: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[2];
        if self.mask & MASK == MASK {
            unsafe { self.on_changedtick.assume_init_drop() };
        }
        self.on_changedtick
            .write(Function::wrap(changedtick).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn on_detach<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnDetach<'a>) -> Result<(), E> + Unpin,
    >(
        &mut self,
        detach: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[3];
        if self.mask & MASK == MASK {
            unsafe { self.on_detach.assume_init_drop() };
        }
        self.on_detach.write(Function::wrap(detach).into_luaref());
        self.mask |= MASK;

        self
    }

    pub fn on_reload<
        E: 'static + Error,
        F: 'static + NvFn + for<'a> Fn(BufOnReload<'a>) -> Result<(), E> + Unpin,
    >(
        &mut self,
        reload: F,
    ) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[4];
        if self.mask & MASK == MASK {
            unsafe { self.on_reload.assume_init_drop() };
        }
        self.on_reload.write(Function::wrap(reload).into_luaref());
        self.mask |= MASK;

        self
    }
}
