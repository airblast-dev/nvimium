use crate::nvim_types::ThinString;

#[macro_export]
macro_rules! th {
    ($v:vis $id:ident = $s:literal) => {
        $v const $id: $crate::nvim_types::ThinString<'static> = $crate::macros::thinstring::th_f(::std::concat!($s, "\0"));
    };
    ($s:literal) => {
        $crate::macros::thinstring::th_f(::std::concat!($s, "\0"))
    }
}

pub(crate) const fn th_f<'a>(s: &'a str) -> ThinString<'a> {
    ThinString::from_null_terminated(s.as_bytes())
}
