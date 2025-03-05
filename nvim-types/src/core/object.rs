use std::fmt::Debug;

use crate::{array::Array, dictionary::Dictionary};

use super::{
    buffer::Buffer, string::OwnedThinString, tab_page::TabPage, window::Window, Boolean, Float,
    Integer,
};

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
    String(OwnedThinString),
    Array(Array),
    Dict(Dictionary),
    LuaRef,
    Buffer(Buffer),
    Window(Window),
    TabPage(TabPage),
}

impl Object {
    pub fn as_bool(self) -> Option<Boolean> {
        match self {
            Object::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_int(self) -> Option<Integer> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_float(self) -> Option<Float> {
        match self {
            Object::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_string(self) -> Option<OwnedThinString> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(self) -> Option<Array> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_dict(self) -> Option<Dictionary> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    // TODO: complete this after adding lua integration
    #[doc(hidden)]
    fn as_luaref(self) -> Option<()> {
        todo!()
    }

    pub fn as_buffer(self) -> Option<Buffer> {
        match self {
            Self::Buffer(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_window(self) -> Option<Window> {
        match self {
            Self::Window(w) => Some(w),
            _ => None,
        }
    }

    pub fn as_tabpage(self) -> Option<TabPage> {
        match self {
            Self::TabPage(t) => Some(t),
            _ => None,
        }
    }
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
            Object::LuaRef => todo!(),
            Object::Buffer(buf) => write!(f, "{:?}", buf),
            Object::Window(win) => write!(f, "{:?}", win),
            Object::TabPage(tp) => write!(f, "{:?}", tp),
        }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        todo!("clone object")
    }
}
