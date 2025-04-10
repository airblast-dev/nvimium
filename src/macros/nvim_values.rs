#[macro_export]
macro_rules! dict {
    ($( $key:literal = $val:tt $(: $kind:tt)?),*) => {
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

#[macro_export]
macro_rules! array {
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
        unsafe { ObjectRef::new(ObjectTag::Bool, &BOOL) }
    }};
    ([$($val:tt $(: $kind:tt)?),*]) => {{
        use $crate::nvim_types::{ Array, object::{ObjectRef, ObjectTag} };
        const ARRAY: &Array = $crate::array![$($val $(: $kind)?),*];

        unsafe { ObjectRef::new(ObjectTag::Array, ARRAY) }
    }};
    ({$($key:literal = $val:tt),*}) => {
        $crate::dict!($($key = $val),*)
    };
    ($string:tt) => {{
        use $crate::nvim_types::{
            ThinString,
            object::{ObjectRef, ObjectTag},
        };
        const IS_STR: &str = $string;
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
    ($odd:tt $(, $a:tt, $b:tt),*) => { ($crate::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt, $even:tt),*) => { $crate::count_tts!($($a)*) << 1 };
}
