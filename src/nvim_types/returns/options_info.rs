use std::ops::Deref;

use crate::nvim_types::{Boolean, Channel, Dict, Integer, KVec, Object, OwnedThinString};

use super::utils::skip_drop_remove_keys;

#[derive(Clone, Debug)]
pub struct OptionsInfo {
    pub options: KVec<OptionInfo>,
}

impl OptionsInfo {
    pub fn from_c_func_ret(d: &mut Dict) -> Self {
        let mut kv = KVec::with_capacity(d.len());
        kv.extend(d.iter_mut().map(|kv| {
            let Object::Dict(d) = &mut kv.object else {
                panic!();
            };

            OptionInfo::from_c_func_ret(d)
        }));

        Self { options: kv }
    }
}

#[derive(Clone, Debug)]
pub struct OptionInfo {
    pub name: OwnedThinString,
    pub shortname: OwnedThinString,
    pub scope: OptionInfoScope,
    pub global_local: Boolean,
    pub commalist: Boolean,
    pub flaglist: Boolean,
    pub was_set: Boolean,
    pub last_set_sid: Integer,
    pub last_set_linenr: Integer,
    pub last_set_chan: Channel,
    pub kind: OptionType,
    pub default: Object,
    pub allows_duplicates: Boolean,
}

impl OptionInfo {
    pub fn from_c_func_ret(d: &mut Dict) -> Self {
        let [
            name,
            shortname,
            scope,
            global_local,
            commalist,
            flaglist,
            was_set,
            last_set_sid,
            last_set_linenr,
            last_set_chan,
            kind,
            default,
            allows_duplicates,
        ] = skip_drop_remove_keys(
            d,
            &[
                "name",
                "shortname",
                "scope",
                "global_local",
                "commalist",
                "flaglist",
                "was_set",
                "last_set_sid",
                "last_set_linenr",
                "last_set_chan",
                "type",
                "default",
                "allows_duplicates",
            ],
            None,
        )
        .unwrap();

        let name = name.as_string().unwrap().clone();
        let shortname = shortname.as_string().unwrap().clone();
        let scope = {
            let scope = scope.as_string().unwrap();
            match scope.as_thinstr().as_slice() {
                b"buf" => OptionInfoScope::Buffer,
                b"win" => OptionInfoScope::Win,
                b"global" => OptionInfoScope::Global,
                _ => unreachable!(),
            }
        };
        let global_local = global_local.as_bool().unwrap();
        let commalist = commalist.as_bool().unwrap();
        let flaglist = flaglist.as_bool().unwrap();
        let was_set = was_set.as_bool().unwrap();

        let last_set_sid = last_set_sid.as_int().unwrap();
        let last_set_linenr = last_set_linenr.as_int().unwrap();
        let last_set_chan = Channel::new(last_set_chan.as_int().unwrap());
        let kind = {
            let kind = kind.as_string().unwrap();
            match kind.as_thinstr().as_slice() {
                b"nil" => OptionType::Nil,
                b"boolean" => OptionType::Boolean,
                b"number" => OptionType::Number,
                b"string" => OptionType::String,
                _ => unreachable!(),
            }
        };
        let default = default.deref().clone();
        let allows_duplicates = allows_duplicates.as_bool().unwrap();

        Self {
            name,
            shortname,
            scope,
            global_local,
            commalist,
            flaglist,
            was_set,
            last_set_sid,
            last_set_linenr,
            last_set_chan,
            kind,
            default,
            allows_duplicates,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OptionInfoScope {
    Buffer,
    Win,
    Global,
}

#[derive(Clone, Copy, Debug)]
pub enum OptionType {
    Nil,
    Boolean,
    Number,
    String,
}
