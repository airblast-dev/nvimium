use crate::nvim_types::ThinString;

macro_rules! mode {
    ($(#[$meta:meta])* $ident:ident, $lit:literal) => {
        $(#[$meta])*
        pub const $ident: KeyMapMode =
            KeyMapMode(ThinString::from_null_terminated($lit.to_bytes_with_nul()));
    };
}

// also in a few other places but this function is the actual check call
// src/nvim/mapping.c 0.10.0 l:973
impl KeyMapMode {
    mode!(EMPTY, c"");
    mode!(INSERT, c"i");
    mode!(CMD_INSERT, c"l");
    mode!(CMD, c"c");
    mode!(NORMAL, c"n");
    mode!(VISUAL_SELECT, c"v");
    mode!(VISUAL, c"x");
    mode!(SELECT, c"s");
    mode!(MODE_OP_PENDING, c"o");
    mode!(MODE_TERMINAL, c"t");

    mode!(
        /// Only accepted when setting a keymap
        ABREVIATION_INSERT, c"ia"
    );
    mode!(
        /// Only accepted when setting a keymap
        ABREVIATION_CMD, c"ca"
    );
    mode!(
        /// Only accepted when setting a keymap
        ABREVIATION_INSERT_CMD, c"!a"
    );
}
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyMapMode(ThinString<'static>);
