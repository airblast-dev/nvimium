use std::fmt::Display;

use super::string::String;

#[derive(Clone, Debug)]
struct Error {
    msg: String,
}

impl Error {
    pub fn new<S>(message: S) -> Self
    where
        String: From<S>,
    {
        Self {
            msg: String::from(message),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.msg.as_ptr()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

impl std::error::Error for Error {}
