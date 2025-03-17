use macros::masked_builder;

use crate::hl_group::HlGroupId;
use crate::object_subs::StringOrInt;
use crate::string::OwnedThinString;
use crate::{Boolean, Integer};

masked_builder! {
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
        #[builder(skip)]
        altfont: Boolean,
        nocombine: Boolean,
        default: Boolean,
        cterm: StringOrInt,
        foreground: StringOrInt,
        #[builder(skip)]
        fg: StringOrInt,
        background: StringOrInt,
        #[builder(skip)]
        bg: StringOrInt,
        ctermfg: StringOrInt,
        ctermbg: StringOrInt,
        special: StringOrInt,
        #[builder(skip)]
        sp: StringOrInt,
        link: HlGroupId,
        #[builder(skip)]
        global_link: HlGroupId,
        fallback: Boolean,
        blend: Integer,
        fg_indexed: Boolean,
        bg_indexed: Boolean,
        force: Boolean,
        // it is unsound to not skip this as neovim will free this if the mask is set
        #[builder(skip)]
        url: OwnedThinString
    }
}
