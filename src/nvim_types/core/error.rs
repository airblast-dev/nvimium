use core::{
    ffi::CStr,
    fmt::{Debug, Display},
};

use libc::{c_char, strlen};

use super::string::{AsThinString, String, ThinString};

// Any platform that uses more than a byte as `c_char` limits the API in a few places.
// TODO: Rather than to limit the API for niche systems find an alternative if possible.
const _: () = assert!(size_of::<u8>() == size_of::<c_char>());

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Error {
    kind: ErrorType,
    msg: *const c_char,
}

impl Debug for Error {
    #[inline(never)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let cs = if !self.msg.is_null() {
            unsafe { CStr::from_ptr(self.msg.cast()) }
        } else {
            c"null ptr"
        };
        write!(f, "{:?}: {:?}", self.kind, cs)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
enum ErrorType {
    None = -1,
    Exception,
    Validation,
}

impl Debug for ErrorType {
    #[inline(never)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var = match self {
            Self::None => "None",
            Self::Exception => "Exception",
            Self::Validation => "Validation",
        };
        f.write_str(var)
    }
}

impl Error {
    #[inline(always)]
    pub(crate) const fn none() -> Self {
        Self {
            kind: ErrorType::None,
            msg: core::ptr::null_mut(),
        }
    }

    pub fn exception(th: ThinString) -> Self {
        let s = String::from(th);
        let ptr = s.as_ptr();
        core::mem::forget(s);
        Self {
            kind: ErrorType::Exception,
            msg: ptr as *const c_char,
        }
    }

    pub fn validation(th: ThinString) -> Self {
        let mut s = Self::exception(th);
        s.kind = ErrorType::Validation;
        s
    }

    pub(crate) fn has_errored(&self) -> bool {
        self.kind != ErrorType::None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl core::error::Error for Error {}
unsafe impl AsThinString for Error {
    fn as_thinstr(&self) -> ThinString<'_> {
        unsafe { ThinString::new(strlen(self.msg), self.msg) }
    }
}
