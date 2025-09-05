macro_rules! nv_str_enum {
    (
        $(#[$($enum_attr:tt)+])*
        $vis:vis enum $enum_name:ident {
        $($enum_variant:ident = $enum_str_val:literal,)*
    }) => {
        $(#[$($enum_attr)+])*
        $vis enum $enum_name {
            $($enum_variant, )*
        }

        impl $enum_name {
            const LOOKUP: &[$crate::nvim_types::ThinString<'static>] = &[$($crate::th!($enum_str_val)),*];
        }

        unsafe impl $crate::nvim_types::AsThinString for $enum_name {
            fn as_thinstr(&self) -> $crate::nvim_types::ThinString<'_> {
                self.as_enum_str()
            }
        }

        impl $enum_name {
            #[inline]
            pub(crate) const fn as_enum_str(&self) -> $crate::nvim_types::ThinString<'static> {
                Self::LOOKUP[*self as usize]
            }
        }
    };
}
pub(crate) use nv_str_enum;

macro_rules! nv_obj_ref_enum {
    (
        $(#[$($enum_attr:tt)+])*
        $vis:vis enum $enum_name:ident {
        $($enum_variant:ident = $enum_val:expr,)*
    }) => {
        $(#[$($enum_attr)+])*
        $vis enum $enum_name {
            $($enum_variant, )*
        }

        impl $enum_name {
            const LOOKUP: &[$crate::nvim_types::object::ObjectRef<'static>] = &[$($enum_val),*];
        }

        impl $enum_name {
            #[inline]
            pub(crate) const fn as_obj_ref(&self) -> $crate::nvim_types::object::ObjectRef<'static> {
                Self::LOOKUP[*self as usize].copied()
            }
        }
    };
}
pub(crate) use nv_obj_ref_enum;

#[cfg(test)]
mod tests {
    use crate::nvim_types::AsThinString;

    nv_str_enum!(
        #[derive(Clone, Copy)]
        enum MyEnum {
            A = "a",
            B = "b",
            C = "c",
        }
    );

    #[test]
    fn my_enum_matches_vars() {
        assert_eq!(MyEnum::LOOKUP[MyEnum::A as usize], "a");
        assert_eq!(MyEnum::LOOKUP[MyEnum::B as usize], "b");
        assert_eq!(MyEnum::LOOKUP[MyEnum::C as usize], "c");

        assert_eq!(MyEnum::A.as_thinstr(), "a");
        assert_eq!(MyEnum::B.as_thinstr(), "b");
        assert_eq!(MyEnum::C.as_thinstr(), "c");

        assert_eq!(MyEnum::A.as_enum_str(), "a");
        assert_eq!(MyEnum::B.as_enum_str(), "b");
        assert_eq!(MyEnum::C.as_enum_str(), "c");
    }
}
