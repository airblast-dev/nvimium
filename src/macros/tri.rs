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

macro_rules! tri_ret {
    (
        $err:ident;
        $call:expr;
        $conv:expr;
    ) => {{
        let mut $err = crate::nvim_types::Error::none();
        let mut result = $call;
        if $err.has_errored() {
            Err($err)
        } else {
            Ok($conv( unsafe { result.assume_init_mut() } ))
        }
    }};
}
pub(crate) use tri_ret;

macro_rules! tri_nc {
    (
        $err:ident;
        $call:expr;
    ) => {{
        let mut $err = crate::nvim_types::Error::none();
        let result = $call;
        if $err.has_errored() {
            Err($err)
        } else {
            Ok(unsafe { result.assume_init() })
        }
    }};
}
pub(crate) use tri_nc;

macro_rules! tri_ez {
    (
        $err:ident;
        $call:expr;
    ) => {{
        let mut $err = crate::nvim_types::Error::none();
        let _: () = $call;
        if $err.has_errored() {
            Err($err)
        } else {
            Ok(())
        }
    }};
}
pub(crate) use tri_ez;

macro_rules! tri_match {
    ($err:ident; $expr:expr; $conv:item; $err_handle:expr) => {{
        let mut $err = crate::nvim_type::Error::none();
        let result = $expr;
        if $err.has_errored() {
            Err($err_handle)
        } else {
            Ok($conv( unsafe { result.assume_init() } ))
        }
    }};
}
pub(crate) use tri_match;
