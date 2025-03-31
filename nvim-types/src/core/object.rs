use std::{fmt::Debug, marker::PhantomData, mem::ManuallyDrop};

use libc::size_t;

use sptr::Strict;

use crate::{array::Array, dictionary::Dictionary};

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

#[repr(u32)]
pub enum ObjectTag {
    Null = 0,
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

// TODO: this sucks, find a way to pin its size without using the magic array whilst supporting different
// types
/// Same as [`Object`] but accepts any type that has a valid layout for a Neovim object
#[doc(hidden)]
#[repr(C)]
pub struct ObjectRef<'a, T> {
    tag: ObjectTag,
    val: [size_t; 3],
    p: PhantomData<&'a T>,
}

impl<T> ObjectRef<'_, T> {
    const SIZE: () = assert!(size_of::<T>() == 16 || size_of::<T>() == 32);
    const OBJ_SIZE: () = assert!(size_of::<ObjectRef<T>>() == size_of::<Object>());

    /// Initialize an [`ObjectRef`] with `T`
    ///
    /// # Safety
    ///
    /// Calling this function requires the tag and value to match the layout of its equal [`Object`]
    pub unsafe fn new(tag: ObjectTag, val: &T) -> Self {
        #[allow(clippy::let_unit_value)]
        let _a = Self::SIZE;
        #[allow(clippy::let_unit_value)]
        let _a = Self::OBJ_SIZE;
        let mut r = ObjectRef {
            tag,
            val: [0; 3],
            p: PhantomData,
        };
        let val = unsafe { (val as *const T).cast::<ManuallyDrop<T>>().read() };
        unsafe { r.val.as_mut_ptr().cast::<ManuallyDrop<T>>().write(val) };
        r
    }
}

impl<'a> From<&'a ThinString<'a>> for ObjectRef<'a, ThinString<'a>> {
    fn from(value: &'a ThinString<'a>) -> Self {
        // This is hand written instead of using ObjectRef::new to be able to test with miri
        // maybe do this for other types as well?
        let addr = value.as_ptr().expose_addr();
        let len = value.len();
        ObjectRef {
            tag: ObjectTag::String,
            val: [addr, len, 0],
            p: PhantomData,
        }
    }
}

impl<'a> From<&'a Array> for ObjectRef<'a, Array> {
    fn from(value: &'a Array) -> Self {
        let addr = value.as_ptr().expose_addr();
        let len = value.len();
        let cap = value.capacity();
        ObjectRef {
            tag: ObjectTag::Array,
            val: [len, cap, addr],
            p: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::mem::{ManuallyDrop, transmute};

    use crate::{array::Array, kvec::KVec, string::ThinString};

    use super::{Object, ObjectRef};

    impl<'a> ObjectRef<'a, ThinString<'a>> {
        fn convert_back(self) -> ThinString<'a> {
            let addr = self.val[0];
            let ptr = sptr::from_exposed_addr(addr);
            unsafe { ThinString::new(self.val[1], ptr) }
        }
    }

    #[test]
    fn object_ref_readback() {
        let th: ThinString<'_> = ThinString::from_null_terminated(b"Hello\0");
        let oref = ObjectRef::from(&th);
        assert_eq!(oref.convert_back(), "Hello");
    }

    #[test]
    fn object_ref_to_object() {
        let obj = Object::from(Array(KVec::with_capacity(10)));
        let arr = Array(KVec::with_capacity(10));
        let obj_ref = ObjectRef::from(&arr);
        let obj_reft = unsafe { transmute::<ObjectRef<_>, ManuallyDrop<Object>>(obj_ref) };
        assert_eq!(&obj, &*obj_reft);
    }
}
