pub mod feedkeys;

use crate::string::ThinString;

macro_rules! fast_th {
    ($ident:ident, $lit:literal) => {
        const $ident: KeyMapMode =
            KeyMapMode(ThinString::from_null_terminated($lit.to_bytes_with_nul()));
    };
}

// also in a few other places but this function is the actual check call
// src/nvim/mapping.c 0.10.0 l:973
impl KeyMapMode {
    fast_th!(INSERT, c"i");
    fast_th!(CMD_INSERT, c"l");
    fast_th!(CMD, c"c");
    fast_th!(NORMAL, c"n");
    fast_th!(VISUAL_SELECT, c"v");
    fast_th!(VISUAL, c"x");
    fast_th!(SELECT, c"s");
    fast_th!(MODE_OP_PENDING, c"o");
    fast_th!(MODE_TERMINAL, c"t");
}
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyMapMode(ThinString<'static>);
