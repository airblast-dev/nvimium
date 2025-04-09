#[doc(hidden)]
#[macro_export]
macro_rules! one_of_objects {
    ($(#[$meta:meta])+ $vis:vis $ident:ident, $($ty:ty),+) => {
        $(#[$meta])*
        #[repr(transparent)]
        $vis struct $ident(Object);
        $(
            impl From<$ty> for $ident {
                fn from(value: $ty) -> $ident {
                    Self(Object::from(value))
                }
            }
        )+

        impl AsRef<Object> for $ident {
            fn as_ref(&self) -> &Object {
                &self.0
            }
        }

        impl ::core::borrow::Borrow<Object> for $ident {
            fn borrow(&self) -> &Object {
                &self.0
            }
        }

        impl From<$ident> for Object {
            fn from(value: $ident) -> Object {
                value.0
            }
        }
    };
}
