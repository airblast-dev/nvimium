// TODO: replace meta fragments with ident to allow better comparison in macros?

#[macro_export]
macro_rules! masked_builder {
    (
        $(#[$meta:meta])*
        $pub:vis struct $ident:ident$(< $( $lf:lifetime ),* >)? {
            $(
                $(#[builder($skip:meta)])?
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
                $vis $field: ::core::mem::MaybeUninit<$field_ty>
            ),*
        }

        impl$(<$($lf),*>)? $ident$(<$($lf),*>),* {
            $crate::func_gen_masked!(
                $(
                    $(#[builder($skip)])?
                    $(
                        #[func_meta = $func_meta]
                    )*
                    $field: $field_ty,
                )*
            );
        }

        impl $(<$($lf),*>)? Default for $ident $(<$($lf),*>)?  {
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }

        impl $(<$($lf),*>)? Drop for $ident $(<$($lf),*>)? {
            fn drop(&mut self) {

                // the first bit is unused so the masks value being 1 or 0 means no fields were set
                if self.mask <= 1 {
                    return;
                }

                // TODO: might be possible to optimize this with tagged scope and comparisons
                let mut base_mask = 1;
                $(
                    base_mask <<= 1;
                    if self.mask & base_mask == base_mask {
                        unsafe { self.$field.assume_init_drop() }
                    }
                )*

            }
        }
    };


}

#[doc(hidden)]
#[macro_export]
macro_rules! func_gen_masked {
    (
        $(#[func_meta = $func_meta:meta])*
        $field:ident: $field_ty:ty,
        $(
            $(#[builder($inner_skip:meta)])?
            $(#[func_meta = $inner_func_meta:meta])*
            $inner:ident: $inner_ty:ty,
        )*
    ) => {
        $(#[$func_meta])*
        pub fn $field<T: Into<$field_ty>>(&mut self, $field: T) -> &mut Self {
            if self.mask & 2 == 2 {
                $crate::masked_builder::cold();
                unsafe { self.$field.assume_init_drop() }
            }
            self.mask |= 2;
            self.$field.write($field.into());
            self
        }
        $crate::func_gen_masked_inner!(
            4,
            $(
                $(#[builder($inner_skip)])?
                $( #[func_meta = $inner_func_meta] )*
                $inner: $inner_ty,
            )*
        );
    };
    (
        #[builder($skip:meta)]
        $field:ident: $field_ty:ty,
        $(
            $(#[builder($inner_skip:meta)])?
            $(#[func_meta = $inner_func_meta:meta])*
            $inner:ident: $inner_ty:ty,
        )*
    ) => {
        $crate::func_gen_masked_inner!(
            4,
            $(
                $(#[builder($inner_skip)])?
                $( #[func_meta = $inner_func_meta] )*
                $inner: $inner_ty,
            )*
        );
    };
    () => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! func_gen_masked_inner {

    (
        $mask:expr,
        #[builder($skip:meta)]
        $field:ident: $field_ty:ty,
        $(
            $( #[builder($inner_skip:meta)] )?
            $( #[func_meta = $inner_func_meta:meta] )*
            $inner:ident: $inner_ty:ty,
        )*
    ) => {
        $crate::func_gen_masked_inner!(
            $mask << 1,
            $(
                $(#[builder($inner_skip)])?
                $(#[func_meta = $inner_func_meta])*
                $inner: $inner_ty,
            )*
        );
    };
    (
        $mask:expr,
        $(#[func_meta = $func_meta:meta])*
        $field:ident: $field_ty:ty,
        $(
            $( #[builder($inner_skip:meta)] )?
            $( #[func_meta = $inner_func_meta:meta] )*
            $inner:ident: $inner_ty:ty,
        )*
    ) => {
        $(#[$func_meta])*
        pub fn $field<T: Into<$field_ty>>(&mut self, $field: T) -> &mut Self {
            if self.mask & $mask == $mask {
                $crate::masked_builder::cold();
                unsafe { self.$field.assume_init_drop() }
            }
            self.mask |= $mask;
            self.$field.write($field.into());
            self
        }
        $crate::func_gen_masked_inner!(
            $mask << 1,
            $(
                $(#[builder($inner_skip)])?
                $(#[func_meta = $inner_func_meta])*
                $inner: $inner_ty,
            )*
        );
    };
    ($mask:expr $(,)?) => {};
}

#[cold]
#[inline(never)]
#[doc(hidden)]
pub fn cold() {}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::{borrow::Cow, mem::MaybeUninit, num::NonZeroUsize};

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
            a: MaybeUninit::new(2),
            b: MaybeUninit::new(""),
            c: MaybeUninit::new(true),
        };
        c.a(20_u32);
        assert_eq!(c.mask, 2);
        c.b("hello");
        assert_eq!(c.mask, 6 | (2 << 1));
        c.c(false);
        assert_eq!(c.mask, 14 | (2 << 2));

        unsafe {
            assert!(!c.c.assume_init());
            assert_eq!(c.a.assume_init(), 20_u32);
            assert_eq!(c.b.assume_init(), "hello");
        }
        masked_builder! {
            struct B<'a> {
                #[func_meta = doc(hidden)]
                a: u32,
                b: u64,
                c: Cow<'a, str>,
            }
        };

        let mut c = B {
            mask: 0,
            a: MaybeUninit::new(1),
            b: MaybeUninit::new(2),
            c: MaybeUninit::zeroed(),
        };

        c.a(5_u32);
        assert_eq!(c.mask, 2);
        c.b(6_u64);
        assert_eq!(c.mask, 2 | 4);
        c.c("HAHAHA".to_owned());
        unsafe {
            assert_eq!(c.c.assume_init_ref(), "HAHAHA");
            assert_eq!(c.mask, 2 | 4 | 8);

            assert_eq!(c.a.assume_init(), 5_u32);
            assert_eq!(c.b.assume_init(), 6_u64);
        }

        B::default();

        masked_builder! {
            struct C {
                a: String,
                b: String,
                c: NonZeroUsize,
            }
        }

        C::default();
    }
}
