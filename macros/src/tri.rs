#[doc(hidden)]
#[macro_export]
macro_rules! tri {
    // patterns are ordered by common use, with the full pattern as a seperate macro to avoid extra
    // matches
    (let mut $err:ident; $expr:expr $(,)?) => {
        $crate::tri_full!($expr, Ok(_ret) => Ok(_ret), Err($err) => Err($err));
    };
    (let mut $err:ident; $expr:expr, Ok($ok:ident) => $okexpr:expr $(,)?) =>  {
        $crate::tri_full!($expr, Ok($ok) => $okexpr, Err($err) => Err($err));
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
