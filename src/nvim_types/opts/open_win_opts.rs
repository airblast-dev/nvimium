use crate::{
    macros::masked_builder::masked_builder,
    nvim_types::{Float, Integer},
};

masked_builder!(
    pub struct OpenWinOpts {
        row: Float,
        col: Float,
        width: Integer,
        height: Integer,
    }
);
