use crate::{
    macros::decl_derive::derive,
    nvim_types::{Boolean, HlGroupId, Integer, OwnedThinString, object_subs::StringOrInt},
};

derive! {
    derive(zeroed_default, masked_builder);
    #[repr(C)]
    pub struct SetHlOpts {
        bold: Boolean,
        standout: Boolean,
        strikethrough: Boolean,
        underline: Boolean,
        undercurl: Boolean,
        underdouble: Boolean,
        underdotted: Boolean,
        underdashed: Boolean,
        italic: Boolean,
        reverse: Boolean,
        #[builder_fn_skip]
        altfont: Boolean,
        nocombine: Boolean,
        default: Boolean,
        cterm: StringOrInt,
        foreground: StringOrInt,
        #[builder_fn_skip]
        fg: StringOrInt,
        background: StringOrInt,
        #[builder_fn_skip]
        bg: StringOrInt,
        ctermfg: StringOrInt,
        ctermbg: StringOrInt,
        special: StringOrInt,
        #[builder_fn_skip]
        sp: StringOrInt,
        link: HlGroupId,
        #[builder_fn_skip]
        global_link: HlGroupId,
        fallback: Boolean,
        blend: Integer,
        fg_indexed: Boolean,
        bg_indexed: Boolean,
        force: Boolean,
        // it is unsound to not skip this as neovim will free this if the mask is set
        #[builder_fn_skip]
        url: OwnedThinString,
    }
}
