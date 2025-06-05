macro_rules! derive {
    (derive($dname:ident $(, $($derive_name:ident),* )?); $($tt:tt)+) => {
        crate::macros::decl_derive::decide_derive!(@to_derive[$dname $(, $($derive_name),*)? ] $($tt)+);
    };
}
pub(crate) use derive;

macro_rules! decide_derive {
    (@to_derive[masked_builder $(, $($next_derive:ident),*)? ] $($tt:tt)+) => {
        crate::macros::masked_builder::masked_builder!($($tt)+);
        crate::macros::decl_derive::decide_derive!(@to_derive[$( $($next_derive),* )?] $($tt)+);
    };
    (@to_derive[builder $(, $($next_derive:ident),*)? ] $($tt:tt)+) => {
        crate::macros::builder!($($tt)+);
        crate::macros::decl_derive::decide_derive!(@to_derive[$( $($next_derive),* )?] $($tt)+);
    };
    (@to_derive[zeroed_default $(, $($next_derive:ident),*)? ] $($tt:tt)+) => {
        crate::macros::zeroed_default::zeroed_default!($($tt)+);
        crate::macros::decl_derive::decide_derive!(@to_derive[$( $($next_derive),* )?]
            $($tt)+
        );
    };
    (@to_derive[] $($tt:tt)+) => {

    };
}

pub(crate) use decide_derive;
