use std::fmt::Debug;

use crate::{array::Array, dictionary::Dictionary, string::ThinString};

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

macro_rules! to_unchecked {
    ( $( $ty:tt, $ident:ident ),+ $(,)?) => {
        $(
            #[inline]
            pub(crate) unsafe fn $ident(self) -> $ty {
                match self {
                    Self::$ty(inner) => inner,
                    _ => ::core::hint::unreachable_unchecked(),
                }
            }
        )+
    };
}

impl Object {
    to_unchecked!(
        Integer,
        into_integer_unchecked,
        Float,
        into_float_unchecked,
        Array,
        into_array_unchecked,
        Buffer,
        into_buffer_unchecked,
        Window,
        into_window_unchecked,
        TabPage,
        into_tabpage_unchecked,
    );

    #[inline]
    pub(crate) unsafe fn into_string_unchecked(self) -> OwnedThinString {
        match self {
            Self::String(s) => s,
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    #[inline]
    pub(crate) unsafe fn into_dict_unchecked(self) -> Dictionary {
        match self {
            Self::Dict(d) => d,
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    #[inline]
    pub(crate) unsafe fn into_bool_unchecked(self) -> Boolean {
        match self {
            Self::Bool(d) => d,
            _ => unsafe { core::hint::unreachable_unchecked() },
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
