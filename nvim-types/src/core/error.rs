use std::{ffi::CStr, fmt::{Debug, Display}};

use super::string::{String, ThinString};

#[derive(Clone)]
#[repr(C)]
pub struct Error {
    kind: ErrorType,
    msg: *const u8,
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cs = unsafe { CStr::from_ptr(self.msg.cast()) };
        write!(f, "{:?}: {:?}", self.kind, cs)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i64)]
enum ErrorType {
    None = -1,
    Exception,
    Validation,
}

impl Error {
    #[inline(always)]
    pub const fn none() -> Self {
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
            msg: ptr,
        }
    }

    pub fn validation(th: ThinString) -> Self {
        let mut s = Self::exception(th);
        s.kind = ErrorType::Validation;
        s
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

impl std::error::Error for Error {}
