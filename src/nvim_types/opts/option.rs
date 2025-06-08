use crate::{
    macros::decl_derive::derive,
    nvim_types::{Buffer, ThinString, Window},
    th,
};

derive!(
    derive(masked_builder, zeroed_default);
    #[repr(C)]
    pub struct OptionOpt<'a> {
        #[builder_fn_skip]
        scope: ThinString<'a>,
        win: Window,
        buf: Buffer,
        filetype: ThinString<'a>,
    }
);

impl<'a> OptionOpt<'a> {
    pub fn scope(&mut self, scope: OptionScope) -> &mut Self {
        let scope = match scope {
            OptionScope::Local => th!("local"),
            OptionScope::Global => th!("global"),
        };

        const MASK: u64 = 1 << builder::MASK_OFFSETS[0];
        if self.mask & MASK == MASK {
            unsafe { self.scope.assume_init_drop() };
        }
        self.scope.write(scope);
        self.mask |= MASK;

        self
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OptionScope {
    Local,
    Global,
}
