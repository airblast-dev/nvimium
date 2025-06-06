use core::{fmt::Debug, mem::ManuallyDrop};
use std::ffi::c_void;
use std::mem::{MaybeUninit, transmute};

use libc::memcpy;

use super::{array::Array, dictionary::Dict};

use super::{
    Boolean, Float, Integer,
    borrowed::Borrowed,
    buffer::Buffer,
    lua_ref::LuaRef,
    string::{OwnedThinString, ThinString},
    tab_page::TabPage,
    window::Window,
};

// For layout rules see https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
// Annoyingly isn't in any other official documentation :|
//
// For the enum values see src/nvim/api/private/defs.h 0.10.0 l:93
#[derive(Debug, Default, PartialEq)]
#[repr(C)]
pub enum Object {
    #[default]
    Null,
    Bool(Boolean),
    Integer(Integer),
    Float(Float),
    String(OwnedThinString),
    Array(Array),
    Dict(Dict),
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
            Self::LuaRef(lr) => Self::LuaRef(lr.clone()),
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

    pub fn into_dict(self) -> Option<Dict> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

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

    pub fn as_bool(&self) -> Option<Boolean> {
        if let Object::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<Integer> {
        if let Object::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<Float> {
        if let Object::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&OwnedThinString> {
        if let Object::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        if let Object::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }

    pub fn as_dict(&self) -> Option<&Dict> {
        if let Object::Dict(d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn as_luaref(&self) -> Option<&LuaRef> {
        if let Object::LuaRef(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_buffer(&self) -> Option<Buffer> {
        if let Object::Buffer(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_window(&self) -> Option<Window> {
        if let Object::Window(w) = self {
            Some(*w)
        } else {
            None
        }
    }

    pub fn as_tabpage(&self) -> Option<TabPage> {
        if let Object::TabPage(t) = self {
            Some(*t)
        } else {
            None
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

impl<T> From<T> for Object
where
    OwnedThinString: From<T>,
{
    fn from(value: T) -> Self {
        Self::String(OwnedThinString::from(value))
    }
}

impl From<Array> for Object {
    fn from(value: Array) -> Self {
        Self::Array(value)
    }
}

impl From<Dict> for Object {
    fn from(value: Dict) -> Self {
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

impl TryFrom<Object> for Dict {
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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectTag {
    Null,
    Bool,
    Integer,
    Float,
    String,
    Array,
    Dict,
    LuaRef,
    Buffer,
    Window,
    TabPage,
}

/// Same as [`Object`] but accepts any type that has a valid layout for a Neovim object
#[doc(hidden)]
#[repr(C)]
pub struct ObjectRef<'a> {
    pub tag: ObjectTag,
    pub(crate) val: ObjectRefVal<'a>,
}

impl<'a> PartialEq for ObjectRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
            && unsafe {
                match self.tag {
                    ObjectTag::Null => true,
                    ObjectTag::Bool => self.val.bool == other.val.bool,
                    ObjectTag::Integer => self.val.num == other.val.num,
                    ObjectTag::Float => self.val.float == other.val.float,
                    ObjectTag::String => self.val.string == other.val.string,
                    ObjectTag::Array => self.val.array == other.val.array,
                    ObjectTag::Dict => self.val.dict == other.val.dict,
                    ObjectTag::LuaRef => self.val.lua_ref == other.val.lua_ref,
                    ObjectTag::Buffer => self.val.buffer == other.val.buffer,
                    ObjectTag::Window => self.val.window == other.val.window,
                    ObjectTag::TabPage => self.val.tab_page == other.val.tab_page,
                }
            }
    }
}

impl<'a> Clone for ObjectRef<'a> {
    fn clone(&self) -> Self {
        let mut cloned: MaybeUninit<ObjectRef<'a>> = MaybeUninit::uninit();
        unsafe {
            memcpy(
                (&raw mut cloned) as *mut c_void,
                (self as *const ObjectRef<'a>) as *const c_void,
                size_of::<Self>(),
            );
            cloned.assume_init()
        }
    }
}

impl<'a> ObjectRef<'a> {
    pub(crate) const fn new_bool(n: Boolean) -> Self {
        Self {
            tag: ObjectTag::Bool,
            val: ObjectRefVal { bool: n },
        }
    }
    pub(crate) const fn new_int(n: Integer) -> Self {
        Self {
            tag: ObjectTag::Integer,
            val: ObjectRefVal { num: n },
        }
    }
    pub(crate) const fn new_th(v: ThinString<'a>) -> Self {
        Self {
            tag: ObjectTag::String,
            val: ObjectRefVal { string: v },
        }
    }
}

#[repr(C)]
pub(crate) union ObjectRefVal<'a> {
    nums: [usize; 3],
    pub bool: Boolean,
    pub num: Integer,
    pub float: Float,
    pub string: ThinString<'a>,
    pub array: ManuallyDrop<Array>,
    pub dict: ManuallyDrop<Dict>,
    pub buffer: Buffer,
    pub window: Window,
    pub tab_page: TabPage,
    pub lua_ref: ManuallyDrop<LuaRef>,
}

impl<'a> Debug for ObjectRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Object has the same alignment and size as ObjectRef
        let obj: &Object = unsafe { transmute(self) };
        f.debug_struct("ObjectRef")
            .field("object", obj)
            .finish_non_exhaustive()
    }
}

impl<'a> From<ThinString<'a>> for ObjectRef<'a> {
    fn from(value: ThinString<'a>) -> Self {
        Self {
            tag: ObjectTag::String,
            val: ObjectRefVal { string: value },
        }
    }
}

impl<'a> From<&'a Array> for ObjectRef<'a> {
    fn from(value: &'a Array) -> Self {
        ObjectRef {
            tag: ObjectTag::Array,
            val: ObjectRefVal {
                array: ManuallyDrop::new(unsafe { (value as *const Array).read() }),
            },
        }
    }
}

impl From<LuaRef> for ObjectRef<'static> {
    fn from(value: LuaRef) -> Self {
        Self {
            tag: ObjectTag::LuaRef,
            val: ObjectRefVal {
                lua_ref: ManuallyDrop::new(value),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nvim_types::ThinString;

    use super::ObjectRef;

    impl ObjectRef<'static> {
        fn convert_back(self) -> ThinString<'static> {
            unsafe { self.val.string }
        }
    }

    #[test]
    fn object_ref_readback() {
        const TH: ThinString<'_> = ThinString::from_null_terminated(b"Hello\0");
        let oref = ObjectRef::from(TH);
        assert_eq!(oref.convert_back(), "Hello");
    }
}
