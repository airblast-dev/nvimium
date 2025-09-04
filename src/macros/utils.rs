// https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
#[doc(hidden)]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $(, $a:tt, $b:tt)*) => { ($crate::count_tts!($($a),*) << 1) | 1 };
    ($($a:tt, $even:tt),*) => { $crate::count_tts!($($a),*) << 1 };
}

pub(crate) use count_tts;
