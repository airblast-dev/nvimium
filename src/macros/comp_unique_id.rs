#[doc(hidden)]
#[macro_export]
macro_rules! gen_unique_ish_id {
    () => {
            ::std::concat!(
                "NVIMIUM UNIQUE ID",
                ::std::file!(),
                ::std::line!(),
                ::std::column!(),
                ::std::module_path!(),
                ::std::env!("CARGO_PKG_VERSION"),
                ::std::env!("CARGO_CRATE_NAME"),
                "\0"
            )
            .as_ptr() as *mut ::core::ffi::c_char
    };
}
