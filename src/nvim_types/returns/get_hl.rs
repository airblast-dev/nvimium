use crate::nvim_types::{Boolean, Dict, KVec, KeyValuePair, Object, OwnedThinString, String, ThinString};
// my lord this is ugly
// at least it provides a flexible API and allows for a sane deallocation strategy
// though maybe a closure and a &Dict arg is better? (in practice its a unwrap and condition check
// mess...)
#[derive(Clone, Debug, Default)]
pub struct HighlightGroups {
    pub groups: KVec<HighlightGroup>,
}

impl HighlightGroups {
    pub(crate) fn from_c_func_ret(d: &Dict) -> Self {
        // filter_map removes some size hint information
        // but we expect all of them are something we can process so just preallocate
        let mut kv = KVec::with_capacity(d.len());
        kv.extend(d.iter().filter_map(|KeyValuePair { key, object }| {
            let name = key.clone();
            let Object::Dict(inner_d) = object else {
                return None;
            };
            Some(HighlightGroup {
                name,
                attributes: HighlightAttributes::from_c_func_ret(inner_d),
            })
        }));

        Self { groups: kv }
    }
}

macro_rules! get_bool_or_false {
    ($d:ident, $key:expr) => {
        'mytag: {
            let ret = match $d.get($key) {
                Some(Object::Bool(b)) => *b,
                // we dont really expect this to happen but in the rare case it does, consider it
                // truthy
                Some(_) => true,
                None => false,
            };
            break 'mytag ret;
        }
    };
}

macro_rules! gen_funcs {
    ($name:ident, $($names:ident),+) => {
        gen_funcs!(1, $name, $($names),+);
    };
    ($count:expr, $name:ident) => {
        pub fn $name(&self) -> Boolean {
            self.attr_map & $count == $count
        }

        // The first free unset bit after flags
        #[allow(unused)]
        const LAST_SET: u64 = $count << 1;

    };
    ($count:expr, $name:ident, $($names:ident),*) => {
        pub fn $name(&self) -> Boolean {
            self.attr_map & $count == $count
        }

        gen_funcs!($count << 1, $($names),*);
    };
}

macro_rules! collect_flags {
    ($impl_for:ident, $func_name:ident, $d:ident, $($key:ident),+) => {
        fn $func_name(d: &Dict) -> u64 {
            let mut shift: u64 = u64::MAX;
            let mut attrs = 0;
            $(
                attrs |= (get_bool_or_false!(d, (stringify!($key))) as u64) << { shift = shift.wrapping_add(1); shift };
            )+
            attrs
        }

        impl $impl_for {
            gen_funcs!($($key),+);
        }
    };
}

collect_flags!(
    HighlightAttributes,
    collect_attributes,
    d,
    default,
    reverse,
    bold,
    italic,
    underline,
    undercurl,
    underdouble,
    underdotted,
    underdashed,
    standout,
    strikethrough,
    altfont,
    nocombine
);

type AttributeMap = u64;

#[derive(Clone, Debug)]
pub struct HighlightGroup {
    pub name: OwnedThinString,
    pub attributes: HighlightAttributes,
}

#[derive(Clone, Debug)]
pub struct HighlightAttributes {
    link: OwnedThinString,
    attr_map: AttributeMap,
    cterm_attributes: HighlightCtermAttributes,
    foreground: [u8; 3],
    background: [u8; 3],
    special: [u8; 3],
    blend: u8,
}

impl HighlightAttributes {
    pub fn foreground(&self) -> Option<[u8; 3]> {
        if self.attr_map & Self::LAST_SET == Self::LAST_SET {
            Some(self.foreground)
        } else {
            None
        }
    }

    pub fn background(&self) -> Option<[u8; 3]> {
        if self.attr_map & ( Self::LAST_SET << 1 ) == Self::LAST_SET << 1 {
            Some(self.background)
        } else {
            None
        }
    }

    pub fn special(&self) -> Option<[u8; 3]> {
        if self.attr_map & ( Self::LAST_SET << 2 ) == Self::LAST_SET << 2 {
            Some(self.special)
        } else {
            None
        }
    }

    pub fn blend(&self) -> Option<u8> {
        if self.attr_map & (Self::LAST_SET << 3) == Self::LAST_SET << 3 {
            Some(self.blend)
        } else {
            None
        }
    }

    pub fn link(&self) -> Option<ThinString<'_>> {
        if self.attr_map & (Self::LAST_SET << 4) == Self::LAST_SET << 4 {
            Some(self.link.as_thinstr())
        } else {
            None
        }
    }

    pub fn cterm_attributes(&self) -> Option<&HighlightCtermAttributes> {
        if self.attr_map & ( Self::LAST_SET << 5) == Self::LAST_SET << 5 {
            Some(&self.cterm_attributes)
        } else {
            None
        }
    }

    pub(crate) fn from_c_func_ret(d: &Dict) -> Self {
        let mut attr_map = collect_attributes(d);
        let mut last_bit = Self::LAST_SET;
        // similar to how neovims C dicts (not the Dict in this library) we use bit flags to determine if a field
        // exists. each bit means a field was set/found. For boolean values a macro above generates
        // the methods and creates `collect_attributes`. for non bool values we handle them
        // manually.
        let foreground;
        if let Some(Object::Integer(fg)) = d.get(c"foreground") {
            let fg = *fg;
            foreground = [(fg >> 16) as u8, (fg >> 8) as u8, fg as u8];
            attr_map |= last_bit;
        } else {
            foreground = [0; 3];
        }
        last_bit <<= 1;
        let background;
        if let Some(Object::Integer(bg)) = d.get(c"background") {
            let bg = *bg;
            background = [(bg >> 16) as u8, (bg >> 8) as u8, bg as u8];
            attr_map |= last_bit;
        } else {
            background = [0; 3];
        }
        let special;
        last_bit <<= 1;
        if let Some(Object::Integer(sp)) = d.get(c"special") {
            attr_map |= last_bit;
            let rgb = *sp;
            special = [(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8]
        } else {
            special = [0; 3];
        }

        last_bit <<= 1;
        let blend;
        if let Some(Object::Integer(bl)) = d.get(c"blend") {
            attr_map |= last_bit;
            blend = *bl as u8;
        } else {
            blend = 0;
        }

        last_bit <<= 1;
        let link;
        if let Some(Object::String(s)) = d.get(c"link") {
            attr_map |= last_bit;
            link = s.clone();
        } else {
            link = String::default().into();
        }

        last_bit <<= 1;
        let cterm_attributes;
        if let Some(Object::Dict(d)) = d.get(c"cterm") {
            attr_map |= last_bit;
            cterm_attributes = HighlightCtermAttributes::from_c_func_ret(d);
        } else {
            cterm_attributes = HighlightCtermAttributes::default();
        }

        Self {
            attr_map,
            foreground,
            background,
            special,
            blend,
            link,
            cterm_attributes,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HighlightCtermAttributes {
    attr_map: AttributeMap,
}

impl HighlightCtermAttributes {
    fn from_c_func_ret(d: &Dict) -> Self {
        Self {
            attr_map: collect_cterm_attributes(d),
        }
    }
}
collect_flags!(
    HighlightCtermAttributes,
    collect_cterm_attributes,
    d,
    bold,
    underline,
    undercurl,
    underdouble,
    underdotted,
    underdashed,
    strikethrough,
    reverse,
    italic,
    altfont,
    nocombine
);

// TODO: add tests
