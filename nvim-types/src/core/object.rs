use std::fmt::Debug;

use crate::{array::Array, dictionary::Dictionary};

use super::{
    Boolean, Float, Integer, borrowed::Borrowed, buffer::Buffer, lua_ref::LuaRef,
    string::OwnedThinString, tab_page::TabPage, window::Window,
};

// For layout rules see https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
// Annoyingly isn't in any other official documentation :|
//
// For the enum values see src/nvim/api/private/defs.h 0.10.0 l:93
#[derive(Debug, Default)]
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

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Bool(b) => Self::Bool(*b),
            Self::Integer(i) => Self::Integer(*i),
            Self::Float(f) => Self::Float(*f),
            Self::String(s) => Self::String(s.clone()),
            Self::Array(a) => Self::Array(a.clone()),
            Self::Dict(d) => Self::Dict(d.clone()),
            Self::LuaRef(_) => todo!("add proper clone to lua state once lua support is added"),
            Self::Buffer(b) => Self::Buffer(*b),
            Self::Window(w) => Self::Window(*w),
            Self::TabPage(t) => Self::TabPage(*t),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Self::String(s), Self::String(src)) => s.clone_from(src),
            (Self::Array(a), Self::Array(src)) => a.clone_from(src),
            (Self::Dict(d), Self::Dict(src)) => d.clone_from(src),
            (Self::LuaRef(_), _) | (_, Self::LuaRef(_)) => {
                todo!("add proper clone_from to lua state once lua support is added")
            }
            // other variants are copy which wont benefit from clone_from
            (se, src) => *se = src.clone(),
        }
    }
}

impl Object {
    // Discriminant values
    pub(crate) const D_NILL: u32 = 0;
    pub(crate) const D_BOOL: u32 = 1;
    pub(crate) const D_INTEGER: u32 = 2;
    pub(crate) const D_FLOAT: u32 = 3;
    pub(crate) const D_STRING: u32 = 4;
    pub(crate) const D_ARRAY: u32 = 5;
    pub(crate) const D_DICT: u32 = 6;
    pub(crate) const D_LUAREF: u32 = 7;
    pub(crate) const D_BUFFER: u32 = 8;
    pub(crate) const D_WINDOW: u32 = 9;
    pub(crate) const D_TABPAGE: u32 = 10;
    pub fn into_bool(self) -> Option<Boolean> {
        match self {
            Object::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn into_int(self) -> Option<Integer> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    pub fn into_float(self) -> Option<Float> {
        match self {
            Object::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<OwnedThinString> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn into_array(self) -> Option<Array> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn into_dict(self) -> Option<Dictionary> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    // TODO: complete this after adding lua integration
    pub fn into_luaref(self) -> Option<LuaRef> {
        match self {
            Self::LuaRef(lr) => Some(lr),
            _ => None,
        }
    }

    pub fn into_buffer(self) -> Option<Buffer> {
        match self {
            Self::Buffer(b) => Some(b),
            _ => None,
        }
    }

    pub fn into_window(self) -> Option<Window> {
        match self {
            Self::Window(w) => Some(w),
            _ => None,
        }
    }

    pub fn into_tabpage(self) -> Option<TabPage> {
        match self {
            Self::TabPage(t) => Some(t),
            _ => None,
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

impl<'a> From<&'a Object> for Borrowed<'a, Object> {
    fn from(value: &'a Object) -> Self {
        Borrowed::new(value)
    }
}
