/// A macro to easily initialize a [`Dict`]
///
/// Initializing nested dictionaries with many values gets very verbose and hard read.
/// To solve that problem this macro wraps all of that code into a easy to read format that is
/// similar to a initializating a lua table.
///
/// # Example
/// ```no_run
/// # fn my_func() {
/// use nvimium::nvim_funcs::global::get_current_buf;
/// use nvimium::nvim_types::Buffer;
/// use nvimium::dict;
/// let buffer: Buffer = get_current_buf();
/// let raw_buffer_handle: i32 = 0;
/// let example_key = "Hello!";
/// let some_return_val = dict! {
///     // use an already defined variable
///     "an interesting buffer" = buffer,
///     // we can provide raw integer values as buffer window and tabpage values
///     // as long as the type can be converted into the specified kind
///     "buffer to be deleted" = raw_buffer_handle: buffer,
///     // you can use a variable as a key
///     // and nested arrays/dicts are supported
///     example_key = [
///         // we can use any literal or variable that can be turned into an object
///         // these values are not stored as Integer objects but rather each is stored
///         // as a buffer handle window handle and tabpage handle respectively
///         1: buffer,
///         2: window,
///         3: tabpage,
///         {
///             "dict key for a dict inside an array inside a dict" = "some interesting value",
///             "description" = raw_buffer_handle: buffer
///         },
///         "end of the array"
///     ]
/// };
///
/// // now lets create the same dictionary by using functions
/// use nvimium::nvim_types::{Object, Dict, Array, OwnedThinString, KeyValuePair, Window,
/// TabPage, KVec};
/// let my_dict = Dict::from_iter([
///     KeyValuePair {
///         key: "an interesting buffer".into(),
///         object: Object::Buffer(buffer)
///     },
///     KeyValuePair {
///         key: "buffer to be deleted".into(),
///         object: Object::Buffer(Buffer::new(0))
///     },
///     KeyValuePair {
///         key: "Hello!".into(),
///         object: Object::Array(Array::from(KVec::from_iter([
///             Object::Buffer(Buffer::new(1)),
///             Object::Window(Window::new(1)),
///             Object::TabPage(TabPage::new(1)),
///             Object::Dict(Dict::from_iter([
///                 KeyValuePair {
///                     key: "dict key for a dict inside an array inside a dict".into(),
///                     object: Object::String("some interesting value".into()),
///                 },
///                 KeyValuePair {
///                     key: "description".into(),
///                     object: Object::Buffer(Buffer::new(0)),
///                 }
///             ])),
///             Object::String("end of the array".into())
///         ])))
///     }
/// ]);
///
/// assert_eq!(my_dict, some_return_val);
/// # }
/// ```
#[macro_export]
macro_rules! dict {
    ($($key:tt = $val:tt $(: $kind:tt)?),*) => {{
        use $crate::nvim_types::{dictionary::{Dict, KeyValuePair}, OwnedThinString, KVec};
        let count = $crate::count_tts!($($key),*);
        let mut kv = KVec::with_capacity(count);
        $(
            let key = OwnedThinString::from($key);
            let object = $crate::to_value!($val $(: $kind)?);
            unsafe {
                kv.push_unchecked( KeyValuePair {
                    key,
                    object,
                });
            }
        )*
        Dict::from(kv)
    }};
}

/// A macro to easily initialize an [`Array`]
///
/// This macro follows the same rules and syntax as [`dict`], see its documentation instead.
#[macro_export]
macro_rules! array {
    ($($val:tt $(: $kind:tt)?),*) => {{
        use $crate::nvim_types::{Array, KVec};
        let count = $crate::count_tts!($($val),*);
        let mut kv = KVec::with_capacity(count);
        $(
            let object = $crate::to_value!($val $(: $kind)?);
            unsafe { kv.push_unchecked(object) };
        )*

        Array::from(kv)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_value {
    (#$val:ident) => {{
        use $crate::nvim_types::Object;
        Object::from($val)
    }};
    (#&$val:ident) => {{
        use $crate::nvim_types::Object;
        Object::from($val.clone())
    }};
    ([$($arr:tt $(: $kind:tt)?),*]) => {{
        use $crate::nvim_types::{ Object, KVec, Array };
        let count = $crate::count_tts!($($arr),*);
        #[allow(unused_mut)]
        let mut kv = KVec::with_capacity(count);
        $(
            let val = $crate::to_value!($arr $(: $kind)?);
            unsafe { kv.push_unchecked(val); }
        )*
        Object::Array(Array::from(kv))
    }};
    ({$($key:tt = $value:tt $(: $kind:tt)?),*}) => {{
        use $crate::nvim_types::{Object, KVec, Dict, dictionary::KeyValuePair, OwnedThinString};
        let count = $crate::count_tts!($($key),*);
        #[allow(unused_mut)]
        let mut kv = KVec::with_capacity(count);
        $(
            unsafe { kv.push_unchecked(
                KeyValuePair {
                    key: OwnedThinString::from($key),
                    object: $crate::to_value!($value $(: $kind)?),
                }
            ) };
        )*
        Object::Dict(Dict::from(kv))
    }};
    ($val:tt $(: $kind:tt)?) => {{
        $crate::decide_literal_kind!($val $(: $kind)?)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! decide_literal_kind {
    ($val:tt: buffer) => {{
        use $crate::nvim_types::{Buffer, HandleT, Object};
        let int = HandleT::from($val);
        Object::Buffer(Buffer::new(int))
    }};
    ($val:tt: window) => {{
        use $crate::nvim_types::{HandleT, Object, Window};
        let int = HandleT::from($val);
        Object::Window(Window::new(int))
    }};
    ($val:tt: tabpage) => {{
        use $crate::nvim_types::{HandleT, Object, TabPage};
        let int = HandleT::from($val);
        Object::TabPage(TabPage::new(int))
    }};
    ($val:tt) => {{
        use $crate::nvim_types::Object;
        Object::from($val)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_value_to_object {
    ($int:tt: int) => {{
        use $crate::nvim_types::{
            Integer,
            object::{ObjectRef, ObjectTag},
        };
        const INT: Integer = $int;
        unsafe { ObjectRef::new(ObjectTag::Integer, &INT) }
    }};
    ($int:tt: buffer) => {{
        use $crate::nvim_types::{
            Integer,
            object::{ObjectRef, ObjectTag},
        };
        const INT: Integer = $int;
        unsafe { ObjectRef::new(ObjectTag::Buffer, &INT) }
    }};
    ($int:tt: window) => {{
        use $crate::nvim_types::{
            Integer,
            object::{ObjectRef, ObjectTag},
        };
        const INT: Integer = $int;
        unsafe { ObjectRef::new(ObjectTag::Window, &INT) }
    }};
    ($int:tt: tabpage) => {{
        use $crate::nvim_types::{
            Integer,
            object::{ObjectRef, ObjectTag},
        };
        const INT: Integer = $int;
        unsafe { ObjectRef::new(ObjectTag::TabPage, &INT) }
    }};
    ($bool:tt: bool) => {{
        use $crate::nvim_types::{
            Boolean,
            object::{ObjectRef, ObjectTag},
        };
        const BOOL: Boolean = $int;
        unsafe { ObjectRef::new(ObjectTag::Bool, &BOOL) }
    }};
    ($float:tt: float) => {{
        use $crate::nvim_types::{
            Float,
            object::{ObjectRef, ObjectTag},
        };
        const FLOAT: Float = $float;
        unsafe { ObjectRef::new(ObjectTag::Float, &FLOAT) }
    }};
    ([$($val:tt $(: $kind:tt)?),*]) => {{
        use $crate::nvim_types::{ Array, object::{ObjectRef, ObjectTag} };
        const ARRAY: &Array = $crate::const_array![$($val $(: $kind)?),*];

        unsafe { ObjectRef::new(ObjectTag::Array, ARRAY) }
    }};
    ({$($key:literal = $val:tt $(: $kind:tt)?),*}) => {{
        use $crate::nvim_types::{ object::{ObjectRef, ObjectTag} };
        unsafe { ObjectRef::new(ObjectTag::Dict, $crate::const_dict!($($key = $val $(: $kind)?),*) ) }
    }};
    (NULL) => {{
        use $crate::nvim_types::{ object::{ObjectRef, ObjectTag} };
        unsafe { ObjectRef::new(ObjectTag::Null, &0) }
    }};
    ($string:literal) => {{
        use $crate::nvim_types::{
            ThinString,
            object::{ObjectRef, ObjectTag},
        };
        const _IS_STR: &str = $string;
        const S: &[u8] = ::std::concat!($string, "\0").as_bytes();
        const TH: ThinString<'static> = ThinString::from_null_terminated(S);
        unsafe { ObjectRef::new(ObjectTag::String, &TH) }
    }};
}

// https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
#[doc(hidden)]
#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $(, $a:tt, $b:tt)*) => { ($crate::count_tts!($($a),*) << 1) | 1 };
    ($($a:tt, $even:tt),*) => { $crate::count_tts!($($a),*) << 1 };
}

#[cfg(test)]
mod value_builders {
    use crate::nvim_types::{
        Array, Buffer, Dict, KVec, KeyValuePair, Object, OwnedThinString, TabPage, Window,
    };

    #[test]
    fn dict() {
        let d = dict!("asd" = 12);
        let expected = Dict::from_iter([KeyValuePair {
            key: OwnedThinString::from("asd"),
            object: Object::Integer(12),
        }]);
        assert_eq!(d, expected);

        let d = dict! {
            "Hello" = "World",
            "MyArray" = []
        };
        let expected = Dict::from_iter([
            KeyValuePair {
                key: "Hello".into(),
                object: Object::String("World".into()),
            },
            KeyValuePair {
                key: "MyArray".into(),
                object: Array::default().into(),
            },
        ]);
        assert_eq!(d, expected);

        let d = dict! {
            "Hello" = {
                "inner dict key" = "inner dict value"
            },
            "second key" = [
                {
                    "Very nested key" = "very nested value"
                }
            ],
            "num" = 12,
            "float" = 0.123,
            "buf" = 12: buffer,
            "win" = 1: window,
            "tabpage" = 3: tabpage
        };

        let expected = Dict::from_iter([
            KeyValuePair {
                key: "Hello".into(),
                object: Object::Dict(Dict::from_iter([KeyValuePair {
                    key: "inner dict key".into(),
                    object: "inner dict value".into(),
                }])),
            },
            KeyValuePair {
                key: "second key".into(),
                object: Object::Array(Array(KVec::from_iter([Object::Dict(Dict::from_iter([
                    KeyValuePair {
                        key: "Very nested key".into(),
                        object: "very nested value".into(),
                    },
                ]))]))),
            },
            KeyValuePair {
                key: "num".into(),
                object: Object::Integer(12),
            },
            KeyValuePair {
                key: "float".into(),
                object: Object::Float(0.123),
            },
            KeyValuePair {
                key: "buf".into(),
                object: Object::Buffer(Buffer::new(12)),
            },
            KeyValuePair {
                key: "win".into(),
                object: Object::Window(Window::new(1)),
            },
            KeyValuePair {
                key: "tabpage".into(),
                object: Object::TabPage(TabPage::new(3)),
            },
        ]);

        assert_eq!(d, expected);
    }

    #[test]
    fn dict_macro_accepts_idents() {
        let s = "Hello World";
        let nes = c"Nested";
        let nested_key = "nested key";
        let nested_value = 12.2;
        let nested_buf = 22;
        let buffer = Buffer::new(1);
        let d = dict! {
            s = 12,
            nes = {
                nested_key = nested_value,
                c"buf handle" = buffer,
                c"buf handle from integer" = nested_buf: buffer
            },
            "asd" = [
                1: buffer
            ]
        };

        let expected = Dict::from_iter([
            KeyValuePair {
                key: "Hello World".into(),
                object: Object::from(12),
            },
            KeyValuePair {
                key: "Nested".into(),
                object: Object::Dict(Dict::from_iter([
                    KeyValuePair {
                        key: "nested key".into(),
                        object: 12.2.into(),
                    },
                    KeyValuePair {
                        key: c"buf handle".into(),
                        object: Object::Buffer(Buffer::new(1)),
                    },
                    KeyValuePair {
                        key: c"buf handle from integer".into(),
                        object: Object::Buffer(Buffer::new(nested_buf)),
                    },
                ])),
            },
            KeyValuePair {
                key: "asd".into(),
                object: Object::Array(Array(KVec::from_iter([Object::Buffer(Buffer::new(1))]))),
            },
        ]);

        assert_eq!(d, expected);
    }
}
