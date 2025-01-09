use std::mem::ManuallyDrop;

use crate::{array::Array, dictionary::Dictionary, string::{String, ThinString}};



// For layout rules see https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
// Annoyingly isn't in any other official documentation :|
//
// For the enum values see src/nvim/api/private/defs.h 0.10.0 l:93
#[derive(Debug, Default)]
#[repr(C, u64)]
pub enum Object {
    #[default]
    Null = 0,
    Bool(bool),
    Integer(i64),
    Float(libc::c_double),
    String(String),
    Array(Array),
    Dict(Dictionary),
    LuaRef,
    Buffer()
}

impl Clone for Object {
    fn clone(&self) -> Self {
        todo!("impl clone for object")
    }
}
