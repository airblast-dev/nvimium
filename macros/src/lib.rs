#[macro_export]
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
               $crate::func_gen!($field: $field_ty);
            )*
        }
    };
}

#[macro_export]
macro_rules! masked_builder {
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
            mask: u64,
            $(
                $(#[$field_meta])*
                $vis $field: $field_ty
            ),*
        }

        impl$(<$($gen),*>)? $ident$(<$($gen),*>),* {
            $crate::func_gen_masked!($($field: $field_ty,)*);
        }
    };
}

/// Creates a default impl using [`core::mem::zeroed`]
///
/// Since many types passed to neovim use a mask to indicate if it has a field, we can just zero
/// initialize them rather than to call default implementations.
///
/// Currently this safe for all neovim primitive types. Still marked as unsafe in case that
/// changes.
#[macro_export]
macro_rules! fast_default {
    (unsafe $ident:ident$(< $( $gen:tt ),* >)?) => {
        impl ::core::default::Default for $ident $(<$($gen),*>)? {
            fn default() -> Self {
                unsafe { ::core::mem::zeroed() }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! func_gen_masked {
    ($field:ident: $field_ty:ty, $($inner:ident: $inner_ty:ty,)*) => {
        pub fn $field<T: Into<$field_ty>>(&mut self, $field: T) -> &mut Self {
            self.mask |= 1;
            self.$field = $field.into();
            self
        }
        $crate::func_gen_masked_inner!(2, $($inner: $inner_ty,)*);
    };
    () => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! func_gen_masked_inner {
    ($mask:expr, $field:ident: $field_ty:ty, $($inner:ident: $inner_ty:ty,)*) => {
        pub fn $field<T: Into<$field_ty>>(&mut self, $field: T) -> &mut Self {
            self.mask |= $mask;
            self.$field = $field.into();
            self
        }
        $crate::func_gen_masked_inner!($mask << 1, $($inner: $inner_ty,)*);
    };
    ($mask:expr $(,)?) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! func_gen {
    ($inner:ident: $inner_ty:ty) => {
        pub fn $inner<T: Into<$inner_ty>>(&mut self, $inner: T) -> &mut Self {
            self.$inner = $inner.into();
            self
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tri {
    // patterns are ordered by common use, with the full pattern as a seperate macro to avoid extra
    // matches
    (let mut $err:ident; $expr:expr $(,)?) => {
        $crate::tri_full!($expr, Ok(_ret) => Ok(()), Err($err) => Err($err));
    };
    (let mut $err:ident; $expr:expr, Ok($ok:ident) => $okexpr:expr $(,)?) =>  {
        $crate::tri_full!($stmt, Ok($ok) => $okexpr, Err($err) => Err($err));
    };
    ($( $tt:tt )+) => {
        $crate::tri_full!($($tt)+)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! tri_full {
    ($expr:expr, Ok($ok:ident) => $okexpr:expr, Err($err:ident) => $errexpr:expr $(,)?) => {
        let mut $err = Error::none();
        let $ok = $expr;
        if $err.has_errored() {
            return { $errexpr };
        }

        return $okexpr
    };
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

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
    fn builder_masked() {
        masked_builder!(
            struct A<'a> {
                a: u32,
                b: &'a str,
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
        assert_eq!(c.mask, 1 << 0);
        c.b("hello");
        assert_eq!(c.mask, 1 | (1 << 1));
        c.c(false);
        assert_eq!(c.mask, 3 | (1 << 2));

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
        assert_eq!(c.mask, 1);
        c.b(6_u64);
        assert_eq!(c.mask, 1 | 2);
        c.c("HAHAHA");
        assert_eq!(c.c, "HAHAHA");
        assert_eq!(c.mask, 1 | 2 | 4);

        assert_eq!(c.a, 5_u32);
        assert_eq!(c.b, 6_u64);
    }
}
