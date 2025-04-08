use nvim_test::test_pkg;

test_pkg!();

mod c_funcs;
mod wrappers;
pub use wrappers::*;

pub(crate) mod internals;
