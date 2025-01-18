use crate::{
    dictionary::Dictionary,
    kvec::KVec,
    object::Object,
    string::{OwnedThinString, ThinString},
    Integer,
};

#[derive(Debug)]
struct EvalStatusLineDict {
    chars: OwnedThinString,
    width: Integer,
    highlights: KVec<HighlightItem>,
}

#[derive(Debug)]
struct HighlightItem {
    start: Integer,
    group: OwnedThinString,
}

impl EvalStatusLineDict {
    pub(crate) fn from_c_func_ret(mut d: Dictionary) -> Self {
        let s = unsafe { d.remove("str").unwrap_unchecked().into_string_unchecked() };
        let width = unsafe {
            d.remove("width")
                .unwrap_unchecked()
                .into_integer_unchecked()
        };
        let highlights = unsafe {
            d.remove("highlights")
                .unwrap_unchecked()
                .into_array_unchecked()
        };

        let h_iter = highlights.into_kvec().into_iter();

        let highlight_items = h_iter
            .map(|ob| {
                let mut d = unsafe { Object::into_dict_unchecked(ob) };
                HighlightItem {
                    start: unsafe {
                        d.remove("start")
                            .unwrap_unchecked()
                            .into_integer_unchecked()
                    },
                    group: unsafe { d.remove("group").unwrap_unchecked().into_string_unchecked() },
                }
            })
            .collect();

        Self {
            chars: s,
            width,
            highlights: highlight_items,
        }
    }
}
