use std::fmt::Debug;

use crate::{array::Array, dictionary::Dictionary, string::ThinString};

use super::{buffer::Buffer, tab_page::TabPage, window::Window, Boolean, Float, Integer};

// For layout rules see https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
// Annoyingly isn't in any other official documentation :|
//
// For the enum values see src/nvim/api/private/defs.h 0.10.0 l:93
#[derive(Default)]
#[repr(C, u32)]
pub enum Object {
    #[default]
    Null = 0,
    Bool(Boolean),
    Integer(Integer),
    Float(Float),
    String(ThinString<'static>),
    Array(Array),
    Dict(Dictionary),
    LuaRef,
    Buffer(Buffer),
    Window(Window),
    TabPage(TabPage),
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Bool(b) => write!(f, "{:?}", b),
            Object::Integer(i) => write!(f, "{:?}", i),
            Object::Float(fl) => write!(f, "{:?}", fl),
            Object::String(th) => write!(f, "{:?}", th),
            Object::Array(a) => write!(f, "{:?}", a),
            Object::Dict(d) => write!(f, "{:?}", d),
            mut a => {
                let ptr: *mut u32 = (&raw mut a).cast();
                write!(f, "Unknown object: {:?}", unsafe { ptr.read() })
            }
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        todo!()
    }
}
