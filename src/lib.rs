use allocator::NvAllocator;
pub use nvim_test;
pub mod nvim_types;
pub use thread_lock;
pub mod macros;
pub mod nvim_funcs;
pub mod allocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: NvAllocator = NvAllocator::new(true);

#[cfg(feature = "testing")]
pub use nvim_test_macro;
#[cfg(all(test, not(miri)))]
nvim_test::test_pkg!();
pub mod plugin;
