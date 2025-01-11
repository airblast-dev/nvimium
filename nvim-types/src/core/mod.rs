pub mod array;
pub mod buffer;
pub mod call_site;
pub mod dictionary;
pub mod error;
pub mod kvec;
pub mod object;
pub mod string;
pub mod tab_page;
pub mod window;

pub type Integer = i64;
pub type Boolean = bool;
pub type Float = libc::c_double;
pub type HandleT = libc::c_int;
