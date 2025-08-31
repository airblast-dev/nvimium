macro_rules! zeroed_default {
    (
        $struct_ty:ty
    ) => {
        impl ::core::default::Default for $struct_ty {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
    };
}

pub(crate) use zeroed_default;
