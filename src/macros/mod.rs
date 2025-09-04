// hidden because the macro is called in user code through a macro
#[doc(hidden)]
pub mod comp_unique_id;
pub(crate) mod constified;
pub(crate) mod hash_face;
pub(crate) mod masked_builder;
pub(crate) mod nv_enum;
pub(crate) mod nvim_values;
pub(crate) mod one_of_objects;
pub(crate) mod thinstring;
pub(crate) mod tri;
pub(crate) mod zeroed_default;
pub(crate) mod utils;

macro_rules! func_gen {
    ($inner:ident: $inner_ty:ty) => {
        pub fn $inner<T: Into<$inner_ty>>(&mut self, $inner: T) -> &mut Self {
            self.$inner = $inner.into();
            self
        }
    };
}
pub(crate) use func_gen;

macro_rules! builder {
    (
        $(#[$meta:meta])*
        $pub:vis struct $ident:ident$(< $( $gen:tt ),* >)? {
            $(
                $(#[$field_meta:meta])*
                $vis:vis $field:ident: $field_ty:ty
            ), *$(,)?
        }
    ) => {
        $(#[$meta])*
        $pub struct $ident$(<$($gen),*>),* {
            $(
                $(#[$field_meta])*
                $vis $field: $field_ty
            ),*
        }

        impl$(<$($gen),*>),* $ident$(<$($gen),*>),* {
            $(
               $crate::macros::func_gen!($field: $field_ty);
            )*
        }
    };
}
pub(crate) use builder;

#[cfg(test)]
mod tests {
    #[test]
    fn builder_lf() {
        builder!(
            struct A<'a> {
                a: usize,
                b: &'a str,
            }
        );

        let mut a = A { a: 1, b: "" };

        a.a(3_u8);
        a.b("b");

        assert_eq!(a.a, 3);
        assert_eq!(a.b, "b");
    }

    #[test]
    fn builder() {
        builder!(
            struct A {
                a: usize,
                b: u32,
            }
        );

        let mut a = A { a: 1, b: 2 };

        a.a(3_u8);
        a.b(2_u32);

        assert_eq!(a.a, 3);
        assert_eq!(a.b, 2);
    }
}
