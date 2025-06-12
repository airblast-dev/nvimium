#[doc(hidden)]
#[macro_export]
macro_rules! gen_unique_ish_id {
    () => {
        $crate::nvim_types::ThinString::from_null_terminated(
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
            .as_bytes(),
        )
    };
}
