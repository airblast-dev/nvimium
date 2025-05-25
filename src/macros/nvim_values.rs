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
        let count = $crate::count_tts!($($key),*);
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

#[allow(unused)]
use crate::nvim_types::Dict;
/// A macro for initializing a const [`Dict`]
///
/// In some cases creating a large dict with multiple [`Object`]'s can be cumbersome and costly.
/// This macro initializes a const [`Dict`] and returns a `'static` reference to it.
///
/// Generally [`dict`] should be preferred as it provides more flexibility when initializing the
/// dictionary. This macro is only recommended in tests or when returning fully const values from
/// a callback.
///
/// # Example
///
/// ```no_run
/// use nvimium::const_dict;
/// use nvimium::nvim_types::Dict;
/// use nvimium::nvim_funcs::global::exec_lua;
///
/// fn my_callback() {
///     const LUA_EXEC_ARGUMENTS: &Dict = const_dict!{
///         "Key" = "Value",
///         // the actual variant of the objects that use integers must be specified as buffers
///         // windows and tabpages also use integers
///         "some number" = 12: int,
///         "a very interesting buffer" = 0: buffer,
///         "woosh" = 1: window,
///         "cool" = 2: tabpage,
///         "nice, we can nest dictionaries and arrays" = {
///             "Dict key value" = [
///                 "We can also create nested array's"
///             ]
///         }
///     };
///     
///     // use the const Dict or return it from a callback
/// }
/// ```
#[macro_export]
macro_rules! const_dict {
    ($( $key:tt = $val:tt $(: $kind:tt)?),*) => {
        {
            use $crate::nvim_types::{ThinString, core::borrowed::Borrowed, Dict, object::{ObjectRef}};
            #[repr(C)]
            struct Kv {
                key: ThinString<'static>,
                obj: ObjectRef,
            }

            #[repr(C)]
            struct D {
                cap: usize,
                len: usize,
                data: *const Kv,
            }
            const COUNT: usize = $crate::count_tts!($($key),*);
            const ARR: [::core::mem::MaybeUninit<Kv>; COUNT] = {
                use core::mem::MaybeUninit;
                #[allow(unused_mut)]
                let mut arr: [MaybeUninit<Kv>; COUNT] = [const { MaybeUninit::zeroed() }; COUNT];
                #[allow(unused)]
                let mut _i = 0;
                $(
                    {
                        const KEY: ThinString<'static> = ThinString::from_null_terminated(::std::concat!($key, "\0").as_bytes());
                        const OBJ: ObjectRef = $crate::const_value_to_object!($val $(: $kind)?);
                        const KV: Kv = Kv {
                            key: KEY,
                            obj: OBJ,
                        };

                        arr[_i] = MaybeUninit::new(KV);
                        _i += 1;
                    }
                )*
                arr
            };
            const B: Borrowed<'static, Dict> = unsafe {
                ::core::mem::transmute::<D, Borrowed<'static, Dict>>(D {
                    len: COUNT,
                    cap: COUNT,
                    data: ARR.as_ptr() as *const Kv
                })
            };

            B.as_ref()
        }

    };
}

#[allow(unused)]
use crate::nvim_types::{Array, Object};
/// A macro for initializing a const [`Array`]
///
/// In some cases creating a large array with multiple [`Object`]'s can be cumbersome and costly.
/// This macro initializes a const [`Array`] and returns a `'static` reference to it.
///
/// # Example
///
/// ```no_run
/// use nvimium::const_array;
/// use nvimium::nvim_types::Array;
/// use nvimium::nvim_funcs::global::exec_lua;
///
/// fn my_callback() {
///     const LUA_EXEC_ARGUMENTS: &Array = const_array![
///         "MyFirstArgument",
///         // the actual variant of the objects that use integers must be specified as buffers
///         // windows and tabpages also use integers
///         12: int,
///         0: buffer,
///         1: window,
///         2: tabpage,
///         NULL,
///         {
///             "Dict key value" = [
///                 "We can also create nested array's"
///             ]
///         }
///     ];
///     
///     // we can now pass these without allocating
///     exec_lua(c"vim.print(...)", LUA_EXEC_ARGUMENTS).unwrap();
/// }
/// ```
#[macro_export]
macro_rules! const_array {
    ($($val:tt $(: $kind:tt)?),*) => {
        {
            use core::mem::MaybeUninit;
            use $crate::nvim_types::{ object::ObjectRef, borrowed::Borrowed, Array };
            const COUNT: usize = $crate::count_tts!($($val),*);
            const ARR: [ObjectRef; COUNT] = {
                #[allow(unused_mut)]
                let mut arr: [MaybeUninit<ObjectRef>; COUNT] = [const { MaybeUninit::<ObjectRef>::uninit() };COUNT];
                let mut _i = 0;
                $(
                    stringify!($val);
                    stringify!($($kind)?);
                    arr[_i] = MaybeUninit::new($crate::const_value_to_object!($val $(: $kind)?));
                    _i += 1;
                )*
                unsafe { ::core::mem::transmute::<[MaybeUninit<ObjectRef>; COUNT], [ObjectRef; COUNT]>(arr) }
            };
            #[repr(C)]
            struct Arr {
                cap: usize,
                len: usize,
                data: *const ObjectRef,
            }
            const ARRAY: Borrowed<'static, Array> = unsafe {
                ::core::mem::transmute::<Arr, Borrowed<'static, Array>>(
                    Arr {
                        cap: COUNT,
                        len: COUNT,
                        data: ARR.as_ptr(),
                    }
                )
            };

            ARRAY.as_ref()
        }
    };
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
mod const_macros {
    use crate::nvim_types::{
        Array, Buffer, Dict, KVec, KeyValuePair, Object, OwnedThinString, Window,
    };

    #[test]
    fn array() {
        const ARR1: &Array = const_array![1: int, 2: int, 3: int];
        let exp = Array(KVec::from_iter([1, 2, 3].map(Object::from)));
        assert_eq!(&exp, ARR1);

        const ARR2: &Array = const_array![
            1: int, 2: int, 3: int, 2: int, "Hello", 31: buffer, 31.5: float,
            ["Bye", 1: int, 2: int],
            {"MyKey" = "MyValue", "apples" = ["InnerArray"]}
        ];
        let exp = Array(KVec::from_iter([
            Object::from(1),
            Object::from(2),
            Object::from(3),
            Object::from(2),
            Object::from(OwnedThinString::from("Hello")),
            Object::Buffer(Buffer::new(31)),
            Object::Float(31.5),
            Object::Array(Array(KVec::from_iter([
                Object::from(OwnedThinString::from("Bye")),
                Object::from(1),
                Object::from(2),
            ]))),
            Object::Dict(Dict::from_iter([
                KeyValuePair::from(("MyKey", Object::String(OwnedThinString::from("MyValue")))),
                KeyValuePair::from((
                    "apples",
                    Object::Array(Array(KVec::from_iter([Object::String(
                        OwnedThinString::from("InnerArray"),
                    )]))),
                )),
            ])),
        ]));
        assert_eq!(&exp, ARR2);
    }

    #[test]
    fn const_dict() {
        const DICT1: &Dict = const_dict! {
            "Hello" = "Bye",
            "MyArray" = [1: int, 2: int, "InsideMyArray"],
            "NestedDict" = {
                "NestedDictKey" = "NestedDictValue",
                "NestedDictStuff" = 12: int
            },
            "buffer" = 0: buffer,
            "window" = 1: window,
            "nothing" = NULL
        };
        let exp = Dict::from_iter([
            KeyValuePair::from(("Hello", Object::String(OwnedThinString::from("Bye")))),
            KeyValuePair::from((
                "MyArray",
                Object::Array(Array(KVec::from_iter([
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::String(OwnedThinString::from("InsideMyArray")),
                ]))),
            )),
            KeyValuePair::from((
                "NestedDict",
                Object::Dict(Dict::from_iter([
                    KeyValuePair::from((
                        "NestedDictKey",
                        Object::String(OwnedThinString::from("NestedDictValue")),
                    )),
                    KeyValuePair::from(("NestedDictStuff", Object::Integer(12))),
                ])),
            )),
            KeyValuePair::from(("buffer", Object::Buffer(Buffer::new(0)))),
            KeyValuePair::from(("window", Object::Window(Window::new(1)))),
            KeyValuePair::from(("nothing", Object::Null)),
        ]);

        assert_eq!(DICT1, &exp);
    }
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
