#[macro_export]
macro_rules! masked_builder {
    (
        $(#[$meta:meta])*
        $pub:vis struct $ident:ident$(< $( $lf:lifetime ),* >)? {
            $(
                $(#[field_meta = $field_meta:meta])*
                $(#[func_meta = $func_meta:meta])*
                $vis:vis $field:ident: $field_ty:ty
            ), *$(,)?
        }
    ) => {
        $(#[$meta])*
        $pub struct $ident$(<$($lf),*>),* {
            mask: u64,
            $(
                $(#[$field_meta])*
                $vis $field: $field_ty
            ),*
        }

        impl$(<$($lf),*>)? $ident$(<$($lf),*>),* {
            $crate::func_gen_masked!($( $( #[$func_meta] )* $field: $field_ty,)*);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! func_gen_masked {
    (
        $(#[$func_meta:meta])* $field:ident: $field_ty:ty,
        $($(#[$inner_meta:meta])* $inner:ident: $inner_ty:ty,)*
    ) => {
        $(#[$func_meta])*
        pub fn $field<T: Into<$field_ty>>(&mut self, $field: T) -> &mut Self {
            self.mask |= 2;
            self.$field = $field.into();
            self
        }
        $crate::func_gen_masked_inner!(4, $( $(#[$inner_meta])* $inner: $inner_ty,)*);
    };
    () => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! func_gen_masked_inner {
    ( 
        $mask:expr, $(#[$func_meta:meta])* $field:ident: $field_ty:ty, 
        $( $(#[$inner_meta:meta])* $inner:ident: $inner_ty:ty,)*
    ) => {
        $( #[$func_meta] )*
        pub fn $field<T: Into<$field_ty>>(&mut self, $field: T) -> &mut Self {
            self.mask |= $mask;
            self.$field = $field.into();
            self
        }
        $crate::func_gen_masked_inner!($mask << 1, $( $(#[$inner_meta])* $inner: $inner_ty,)*);
    };
    ($mask:expr $(,)?) => {};
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    #[test]
    fn builder_masked() {
        masked_builder!(
            struct A<'a> {
                #[func_meta = doc(hidden)]
                a: u32,
                b: &'a str,
                #[func_meta = doc(hidden)]
                c: bool,
            }
        );
        let mut c = A {
            mask: 0,
            a: 2,
            b: "",
            c: true,
        };
        c.a(20_u32);
        assert_eq!(c.mask, 2);
        c.b("hello");
        assert_eq!(c.mask, 6 | (2 << 1));
        c.c(false);
        assert_eq!(c.mask, 14 | (2 << 2));

        assert!(!c.c);
        assert_eq!(c.a, 20_u32);
        assert_eq!(c.b, "hello");
        masked_builder! {
            struct B<'a> {
                a: u32,
                b: u64,
                c: Cow<'a, str>,
            }
        };

        let mut c = B {
            mask: 0,
            a: 1,
            b: 2,
            c: Cow::from("hi!"),
        };

        c.a(5_u32);
        assert_eq!(c.mask, 2);
        c.b(6_u64);
        assert_eq!(c.mask, 2 | 4);
        c.c("HAHAHA");
        assert_eq!(c.c, "HAHAHA");
        assert_eq!(c.mask, 2 | 4 | 8);

        assert_eq!(c.a, 5_u32);
        assert_eq!(c.b, 6_u64);
    }
}
