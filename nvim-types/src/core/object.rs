use std::{any::TypeId, fmt::Debug};

use crate::{array::Array, dictionary::Dictionary};

use super::{
    buffer::Buffer, lua_ref::LuaRef, string::OwnedThinString, tab_page::TabPage, window::Window,
    Boolean, Float, Integer,
};

// For layout rules see https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
// Annoyingly isn't in any other official documentation :|
//
// For the enum values see src/nvim/api/private/defs.h 0.10.0 l:93
#[derive(Clone, Default)]
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
    LuaRef(LuaRef),
    Buffer(Buffer),
    Window(Window),
    TabPage(TabPage),
}

impl Object {
    pub fn to_bool(self) -> Option<Boolean> {
        match self {
            Object::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn to_int(self) -> Option<Integer> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    pub fn to_float(self) -> Option<Float> {
        match self {
            Object::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn to_string(self) -> Option<OwnedThinString> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_array(self) -> Option<Array> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn to_dict(self) -> Option<Dictionary> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    // TODO: complete this after adding lua integration
    #[doc(hidden)]
    fn to_luaref(self) -> Option<()> {
        todo!()
    }

    pub fn to_buffer(self) -> Option<Buffer> {
        match self {
            Self::Buffer(b) => Some(b),
            _ => None,
        }
    }

    pub fn to_window(self) -> Option<Window> {
        match self {
            Self::Window(w) => Some(w),
            _ => None,
        }
    }

    pub fn to_tabpage(self) -> Option<TabPage> {
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
            Object::LuaRef(lref) => write!(f, "{:?}", lref),
            Object::Buffer(buf) => write!(f, "{:?}", buf),
            Object::Window(win) => write!(f, "{:?}", win),
            Object::TabPage(tp) => write!(f, "{:?}", tp),
        }
    }
}

impl From<Boolean> for Object {
    fn from(value: Boolean) -> Self {
        Self::Bool(value)
    }
}

impl From<Integer> for Object {
    fn from(value: Integer) -> Self {
        Self::Integer(value)
    }
}

impl From<Float> for Object {
    fn from(value: Float) -> Self {
        Self::Float(value)
    }
}

impl From<OwnedThinString> for Object {
    fn from(value: OwnedThinString) -> Self {
        Self::String(value)
    }
}

impl From<Array> for Object {
    fn from(value: Array) -> Self {
        Self::Array(value)
    }
}

impl From<Dictionary> for Object {
    fn from(value: Dictionary) -> Self {
        Self::Dict(value)
    }
}

impl From<LuaRef> for Object {
    fn from(value: LuaRef) -> Self {
        Self::LuaRef(value)
    }
}

impl From<Buffer> for Object {
    fn from(value: Buffer) -> Self {
        Self::Buffer(value)
    }
}

impl From<Window> for Object {
    fn from(value: Window) -> Self {
        Self::Window(value)
    }
}

impl From<TabPage> for Object {
    fn from(value: TabPage) -> Self {
        Self::TabPage(value)
    }
}

impl TryFrom<Object> for Boolean {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Bool(b) => Ok(b),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for Integer {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Integer(i) => Ok(i),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for Float {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Float(f) => Ok(f),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for OwnedThinString {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::String(s) => Ok(s),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for Array {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Array(a) => Ok(a),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for Dictionary {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Dict(d) => Ok(d),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for LuaRef {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::LuaRef(d) => Ok(d),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for Buffer {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Buffer(b) => Ok(b),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for Window {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Window(w) => Ok(w),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

impl TryFrom<Object> for TabPage {
    type Error = ObjectConversionError;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::TabPage(v) => Ok(v),
            _ => Err(ObjectConversionError::IncorrectKind),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ObjectConversionError {
    IncorrectKind,
}
