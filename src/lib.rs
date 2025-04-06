pub use nvim_test;
pub mod nvim_types;
use nvim_test::test_pkg;
pub use thread_lock;
pub mod nvim_funcs;

#[cfg(feature = "testing")]
pub use nvim_test_macro;
#[cfg(all(test, not(miri)))]
test_pkg!();
pub mod plugin;
