use allocator::NvAllocator;
pub use nvim_test;
pub mod nvim_types;
pub use thread_lock;
pub mod allocator;
pub mod macros;
pub mod nvim_funcs;

#[global_allocator]
static GLOBAL_ALLOCATOR: NvAllocator = NvAllocator::new(true);

#[cfg(feature = "testing")]
pub use nvim_test_macro;
#[doc(hidden)]
#[cfg(feature = "testing")]
pub mod test_macro_utils;
#[cfg(all(test, not(miri)))]
nvim_test::test_pkg!();
pub mod plugin;
