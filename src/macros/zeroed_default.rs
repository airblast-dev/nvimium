macro_rules! zeroed_default {
    (
        $(#[$( $attrs:tt )+])*
        $vis:vis struct $struct_ident:ident $(<$($lf:lifetime),+>)?
        { $($tt:tt)* }
    ) => {
        impl $(<$($lf),+>)? core::default::Default for $struct_ident $(<$($lf),+>)? {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
    };
}

pub(crate) use zeroed_default;
