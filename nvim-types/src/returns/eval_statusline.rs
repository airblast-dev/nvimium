use std::mem::ManuallyDrop;

use crate::{Integer, array::Array, dictionary::Dictionary, kvec::KVec, string::OwnedThinString};

#[derive(Debug)]
pub struct EvalStatusLineDict {
    pub chars: OwnedThinString,
    pub width: Integer,
    pub highlights: Option<KVec<HighlightItem>>,
}

#[derive(Debug)]
pub struct HighlightItem {
    pub start: Integer,
    pub groups: Array,
}

impl EvalStatusLineDict {
    pub fn from_c_func_ret(mut d: ManuallyDrop<Dictionary>) -> Self {
        let s = d
            .remove_skip_key_drop("str")
            .unwrap()
            .into_string()
            .unwrap();
        let width = d.remove_skip_key_drop("width").unwrap().into_int().unwrap();
        let Some(highlights) = d
            .remove_skip_key_drop("highlights")
            .map(|ob| ob.into_array().unwrap().into_kvec())
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
                let mut d = ob.into_dict().unwrap();
                let start = d.remove_skip_key_drop("start").unwrap().into_int().unwrap();
                let groups = d
                    .remove_skip_key_drop("groups")
                    .unwrap()
                    .into_array()
                    .unwrap();

                // deprecated value
                d.remove_skip_key_drop("group");

                HighlightItem {
                    start,
                    // how long a group value may live is undefined, so we clone the value to an
                    // OwnedThinString to ensure the value can live as long as needed
                    groups,
                }
            })
            .collect();

        Self {
            chars: s,
            width,
            highlights: Some(highlight_items),
        }
    }
}
