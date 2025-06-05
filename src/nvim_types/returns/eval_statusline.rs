use std::ops::{Deref, DerefMut};

use crate::nvim_types::{Array, Dict, Integer, KVec, Object, string::OwnedThinString};

use super::utils::skip_drop_remove_keys;

#[derive(Debug)]
pub struct EvalStatusLine {
    pub chars: OwnedThinString,
    pub width: Integer,
    pub highlights: Option<KVec<HighlightItem>>,
}

#[derive(Debug)]
pub struct HighlightItem {
    pub start: Integer,
    pub groups: Array,
}

impl EvalStatusLine {
    pub fn from_c_func_ret(d: &mut Dict) -> Self {
        let [s, width, mut highlights] = skip_drop_remove_keys(
            d,
            &["str", "width", "highlights"],
            Some(|mk| mk.eq("highlights").then(|| Object::Null)),
        )
        .unwrap();
        let s = if let Object::String(s) = s.deref() {
            s.clone()
        } else {
            panic!();
        };
        let width = if let Object::Integer(width) = width.deref() {
            *width
        } else {
            panic!();
        };
        let highlights = match highlights.deref_mut() {
            Object::Array(h) => h,
            Object::Null => {
                return Self {
                    chars: s,
                    width,
                    highlights: None,
                };
            }
            _ => unreachable!(),
        };

        let highlight_items = highlights
            .iter_mut()
            .map(|mut ob| {
                let Object::Dict(d) = ob.deref_mut() else {
                    unreachable!()
                };
                let [start, groups] = skip_drop_remove_keys(d, &["start", "groups"], None).unwrap();
                let Object::Integer(start) = start.deref() else {
                    unreachable!();
                };
                let start = *start;
                let Object::Array(groups) = groups.deref() else {
                    unreachable!()
                };

                HighlightItem {
                    start,
                    // how long a group value may live is undefined, so we clone the value to an
                    // OwnedThinString to ensure the value can live as long as needed
                    groups: groups.clone(),
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
