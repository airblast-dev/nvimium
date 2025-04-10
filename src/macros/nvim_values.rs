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
                let mut arr: [MaybeUninit<Kv>; COUNT] = [const { MaybeUninit::uninit() }; COUNT];
                #[allow(unused)]
                let mut _i = 0;
                $(
                    {
                        const KEY: ThinString<'static> = ThinString::from_null_terminated(::std::concat!($key, "\0").as_bytes());
                        const OBJ: ObjectRef = $crate::value_to_object!($val $(: $kind)?);
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
                    arr[_i] = MaybeUninit::new($crate::value_to_object!($val $(: $kind)?));
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

#[macro_export]
macro_rules! value_to_object {
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
    ($string:tt) => {{
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
mod array {
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
    fn dict() {
        const DICT1: &Dict = const_dict! {
            "Hello" = "Bye",
            "MyArray" = [1: int, 2: int, "InsideMyArray"],
            "NestedDict" = {
                "NestedDictKey" = "NestedDictValue",
                "NestedDictStuff" = 12: int
            },
            "buffer" = 0: buffer,
            "window" = 1: window
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
        ]);

        assert_eq!(DICT1, &exp);
    }
}
