use crate::{dictionary::Dictionary, kvec::KVec, object::Object, string::OwnedThinString, Integer};

#[derive(Debug)]
pub struct EvalStatusLineDict {
    pub chars: OwnedThinString,
    pub width: Integer,
    pub highlights: Option<KVec<HighlightItem>>,
}

#[derive(Debug)]
pub struct HighlightItem {
    pub start: Integer,
    pub group: OwnedThinString,
}

impl EvalStatusLineDict {
    pub fn from_c_func_ret(mut d: Dictionary) -> Self {
        let s = d.remove_skip_key_drop("str").unwrap().as_string().unwrap();
        let width = d.remove_skip_key_drop("width").unwrap().as_int().unwrap();
        let Some(highlights) = d
            .remove_skip_key_drop("highlights")
            .map(|ob| ob.as_array().unwrap().into_kvec())
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
                let mut d = ob.as_dict().unwrap();
                let start = d.remove_skip_key_drop("start").unwrap().as_int().unwrap();
                let group = d
                    .remove_skip_key_drop("group")
                    .unwrap()
                    .as_string()
                    .unwrap();
                let hi = HighlightItem {
                    start,
                    // how long a group value may live is undefined, so we clone the value to an
                    // OwnedThinString to ensure the value can live as long as needed
                    group: group.clone(),
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
