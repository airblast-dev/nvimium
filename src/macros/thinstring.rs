#[macro_export]
macro_rules! th {
    ($v:vis $id:ident = $s:literal) => {
        $v const $id: $crate::nvim_types::ThinString<'static> = $crate::nvim_types::ThinString::from_null_terminated(::std::concat!($s, "\0").as_bytes());
    };
    ($s:literal) => {
        $crate::nvim_types::ThinString::from_null_terminated(::std::concat!($s, "\0").as_bytes())
    }
}
