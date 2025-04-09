#[macro_export]
macro_rules! dict {
    ($($key:literal = $val:tt,),*) => {
        {
            use $crate::nvim_types::{ThinString, Object, core::borrowed::Borrowed, Dict};
            #[repr(C)]
            struct Kv {
                key: ThinString<'static>,
                obj: Borrowed<'static, Object>,
            }

            #[repr(C)]
            struct D {
                cap: usize,
                len: usize,
                data: *const Kv,
            }
            const ARR_LEN: usize = $crate::count_tts!($($key)*);
            const ARR: [::core::mem::MaybeUninit<Kv>; ARR_LEN] = {
                use core::mem::MaybeUninit;
                let mut arr: [MaybeUninit<Kv>; ARR_LEN] = [const { MaybeUninit::uninit() }; ARR_LEN];
                #[allow(unused)]
                let mut _i = 0;
                $(
                    {
                        const KEY: ThinString<'static> = ThinString::from_null_terminated($key.as_bytes());
                        const NULL: Object = Object::Null;
                        const VAL: Borrowed<'static, Object> = Borrowed::new(&NULL);
                        const KV: Kv = Kv {
                            key: KEY,
                            obj: VAL,
                        };

                        arr[_i] = MaybeUninit::new(KV);
                        _i += 1;
                    }
                )*
                arr
            };
            unsafe {
                ::core::mem::transmute::<D, Borrowed<'static, Dict>>(D {
                    len: ARR.len(),
                    cap: ARR.len(),
                    data: ARR.as_ptr() as *const Kv
                })
            }
        }

    };
}

// https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
#[doc(hidden)]
#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { ($crate::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}
