use crate::{dictionary::Dictionary, kvec::KVec, object::Object, string::OwnedThinString, Integer};

#[derive(Debug)]
pub struct EvalStatusLineDict {
    chars: OwnedThinString,
    width: Integer,
    highlights: Option<KVec<HighlightItem>>,
}

#[derive(Debug)]
pub struct HighlightItem {
    start: Integer,
    group: OwnedThinString,
}

impl EvalStatusLineDict {
    pub fn from_c_func_ret(mut d: Dictionary) -> Self {
        let s = unsafe { d.remove("str").unwrap_unchecked().into_string_unchecked() };
        let width = unsafe {
            d.remove("width")
                .unwrap_unchecked()
                .into_integer_unchecked()
        };
        let Some(highlights) = d
            .remove("highlights")
            .map(|ob| unsafe { Object::into_array_unchecked(ob).into_kvec() })
        else {
            return Self {
                chars: s,
                width,
                highlights: None,
            };
        };

        let highlight_items = highlights
            .into_iter()
            .map(|ob| {
                let mut d = unsafe { Object::into_dict_unchecked(ob) };
                let start = unsafe {
                    d.remove("start")
                        .unwrap_unchecked()
                        .into_integer_unchecked()
                };
                let group = unsafe { d.remove("group").unwrap_unchecked().into_string_unchecked() };
                let hi = HighlightItem {
                    start,
                    group: OwnedThinString::from(group.as_thinstr()),
                };
                core::mem::forget(group);
                hi
            })
            .collect();

        Self {
            chars: s,
            width,
            highlights: Some(highlight_items),
        }
    }
}
