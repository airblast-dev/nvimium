// The generics are a bit confusing so heres a quick explanation:
//
// # N
//
// The number of strings to hash.
//
// # SUM_LEN
//
// The sum of the length of all provided strings.
//
// # MAX_LEN
//
// The longest strings length out of the provided strings.
//
//
// We could over allocate in some cases but this makes it easy to cause a stack overflow in some
// cases. Instead the generics are expected to be calculated outside this function, then stored in
// a constant and then passed as a const generic to [`build_buckets`].

/// They key and value like in a Lua table
///
/// The length should actually be a usize but its fairly easy to cause a stack overflow in tests so
/// we store a u16.
///
/// This also means we support up to [`u16::MAX`] fields.
#[derive(Clone, Copy, Debug)]
struct KeyValue<const N: usize> {
    // this is a single ascii char
    //
    // technically can be invalid UTF-8 as well but in this context we are storing a single byte
    // from a structs field name which we already know is only ascii in Neovim
    key: u8,
    len: u16,
    val: [&'static str; N],
}

impl<const N: usize> KeyValue<N> {
    const fn empty() -> Self {
        Self {
            key: 0,
            len: 0,
            val: [""; N],
        }
    }

    const fn append(&mut self, s: &'static str) {
        self.val[self.len as usize] = s;
        self.len += 1;
    }
}

#[derive(Clone, Copy, Debug)]
struct Bucket<const N: usize, const SUM_LEN: usize> {
    len: usize,
    kvs: [KeyValue<N>; SUM_LEN],
}

impl<const N: usize, const SUM_LEN: usize> Bucket<N, SUM_LEN> {
    const fn empty() -> Self {
        Self {
            len: 0,
            kvs: [KeyValue::empty(); SUM_LEN],
        }
    }
    const fn append(&mut self, key: usize, s: &'static str) {
        match self.find_kv_idx(key) {
            Some(kv_idx) => {
                self.kvs[kv_idx].append(s);
            }
            None => {
                let new = self.next();
                new.key = key as u8;
                new.append(s);
                self.len += 1;
            }
        }
    }

    const fn find_kv_idx(&self, key: usize) -> Option<usize> {
        let mut i = 0;
        while i < self.len {
            if self.kvs[i].key == key as u8 {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    const fn next(&mut self) -> &mut KeyValue<N> {
        &mut self.kvs[self.len]
    }

    const fn keys(&self) -> [u8; SUM_LEN] {
        let mut keys = [0; SUM_LEN];
        let mut i = 0;
        while i < self.len {
            keys[i] = self.kvs[i].key;

            i += 1;
        }

        keys
    }
}

#[derive(Clone, Copy, Debug)]
struct LenPos<const N: usize, const SUM_LEN: usize> {
    pos: usize,
    bucket: Bucket<N, SUM_LEN>,
}

impl<const N: usize, const SUM_LEN: usize> LenPos<N, SUM_LEN> {
    const fn empty() -> Self {
        Self {
            pos: 0,
            bucket: Bucket::empty(),
        }
    }
}

#[derive(Debug)]
struct LenPosBuckets<const N: usize, const SUM_LEN: usize, const MAX_LEN: usize> {
    len: usize,
    len_pos: [LenPos<N, SUM_LEN>; MAX_LEN],
}

impl<const N: usize, const SUM_LEN: usize, const MAX_LEN: usize>
    LenPosBuckets<N, SUM_LEN, MAX_LEN>
{
    const fn empty() -> Self {
        Self {
            len: 0,
            len_pos: [LenPos::empty(); MAX_LEN],
        }
    }

    const fn find_pos_bucket(&mut self, pos: usize) -> Option<&mut LenPos<N, SUM_LEN>> {
        let mut i = 0;
        while i < MAX_LEN {
            if self.len_pos[i].pos == pos {
                return Some(&mut self.len_pos[i]);
            }

            i += 1;
        }

        None
    }
}

const fn sort_ints(arr: &mut [u8]) {
    loop {
        let mut i = 1;
        let mut swapped = false;
        while i < arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
            i += 1;
        }

        if !swapped {
            break;
        }
    }
}

const fn extend_arr<T: Copy + std::fmt::Debug>(
    len: &mut usize,
    arr: &mut [T],
    ext_len: usize,
    ext_arr: &[T],
) {
    let mut i = 0;
    let mut start = *len;
    let end = *len + ext_len;
    while start < end {
        arr[start] = ext_arr[i];

        start += 1;
        i += 1;
    }

    *len += ext_len;
}

/// A straight forward implementation of the `build_pos_hash` function.
///
/// Intended to be used in const context as it can use a gigantic amount of stack space.
///
/// Various methods are implemented to make this easier to read.
/// https://github.com/neovim/neovim/blob/6c4ddf607f0b0b4b72c4a949d796853aa77db08f/src/gen/hashy.lua#L15C1-L15C35
const fn build_buckets<const N: usize, const SUM_LEN: usize, const MAX_LEN: usize>(
    strings: &[&'static str; N],
) -> LenPosBuckets<N, SUM_LEN, MAX_LEN> {
    let mut len_buckets: Bucket<N, N> = Bucket::empty();
    let mut i = 0;

    while i < N {
        let s = strings[i];
        len_buckets.append(s.len(), s);
        i += 1;
    }

    let mut len_pos_buckets = LenPosBuckets::<N, SUM_LEN, MAX_LEN>::empty();

    let mut len = 1;
    while len <= MAX_LEN {
        let strs_idx = len_buckets.find_kv_idx(len);
        if let Some(strs_idx) = strs_idx {
            let strs = &len_buckets.kvs[strs_idx];

            let mut best_pos = 0;
            let mut min_size = strs.len * 2;
            let mut best_bucket = Bucket::empty();

            let mut pos = 1;
            while pos <= len {
                let mut try_bucket = Bucket::empty();

                let mut strs_i = 0;
                // for _, str in ipairs(strs) do
                while strs_i < (strs.len as usize) {
                    let s = strs.val[strs_i];
                    if s.len() >= pos {
                        let pos_char = s.as_bytes()[pos - 1];

                        try_bucket.append(pos_char as usize, s);
                    };

                    strs_i += 1;
                }

                let mut max_size = 1;
                let mut pos_strs_i = 0;
                // for _, pos_strs in pairs(try_bucket) do
                while pos_strs_i < SUM_LEN {
                    let l = try_bucket.kvs[pos_strs_i].len;
                    if max_size < l {
                        max_size = l;
                    }

                    pos_strs_i += 1;
                }

                if max_size < min_size {
                    best_pos = pos;
                    min_size = max_size;
                    best_bucket = try_bucket;
                }

                pos += 1;
            }

            let b = &mut len_pos_buckets.len_pos[len - 1];
            b.bucket = best_bucket;
            b.pos = best_pos;
            len_pos_buckets.len += 1;
        }

        len += 1;
    }

    len_pos_buckets
}

const fn sorted_fields_shifts<const N: usize, const SUM_LEN: usize, const MAX_LEN: usize>(
    mut len_pos_buckets: LenPosBuckets<N, SUM_LEN, MAX_LEN>,
) -> [&'static str; N] {
    let mut new_order = [""; N];
    let mut new_order_len = 0;
    let mut len = 1;
    while len <= MAX_LEN {
        let mut vals = &mut len_pos_buckets.len_pos[len - 1];

        let LenPos {
            pos: _pos,
            bucket: pos_buck,
        } = &mut vals;
        // SAFETY: keys returns a nonnull and correctly aligned pointer and at least pos_buck.len items are
        // guaranteed to exist
        //
        // We are required to do this as we have no other way to get a sub slice of an array in
        // const context without unsafe
        let keys =
            unsafe { std::slice::from_raw_parts_mut(pos_buck.keys().as_mut_ptr(), pos_buck.len) };
        sort_ints(keys);

        if pos_buck.len > 1 {
            let mut ci = 0;
            while ci < pos_buck.len {
                let buck = pos_buck.kvs[pos_buck.find_kv_idx(keys[ci] as usize).unwrap()];

                extend_arr(
                    &mut new_order_len,
                    &mut new_order,
                    buck.len as usize,
                    &buck.val,
                );

                ci += 1;
            }
        } else if let Some(b_idx) = pos_buck.find_kv_idx(keys[0] as usize) {
            let b = &pos_buck.kvs[b_idx];
            extend_arr(&mut new_order_len, &mut new_order, 1, &[b.val[0]]);
        }

        len += 1;
    }

    new_order
}

pub const fn strings_len_sum(strings: &[&'static str]) -> usize {
    let mut i = 0;
    let mut sum = 0;
    while i < strings.len() {
        let s = strings[i];

        sum += s.len();

        i += 1;
    }

    sum
}

pub const fn strings_len_max(strings: &[&'static str]) -> usize {
    let mut i = 0;
    let mut max = 0;
    while i < strings.len() {
        let s = strings[i];

        if max < s.len() {
            max = s.len();
        }

        i += 1;
    }

    max
}

pub const fn str_eq(s1: &'static str, s2: &'static str) -> bool {
    s1.len() == s2.len()
        && 'a: {
            let mut i = 0;
            while i < s1.len() {
                if s1.as_bytes()[i] != s2.as_bytes()[i] {
                    break 'a false;
                }

                i += 1;
            }

            true
        }
}

pub const fn fields_to_bit_shifts<const N: usize, const SUM_LEN: usize, const MAX_LEN: usize>(
    strings: &[&'static str; N],
) -> [usize; N] {
    let len_pos_buckets: LenPosBuckets<N, SUM_LEN, MAX_LEN> = build_buckets(strings);
    let reordered = sorted_fields_shifts(len_pos_buckets);

    let mut shifts = [0; N];

    let mut i = 0;
    while i < N {
        let original = strings[i];
        let mut x = i;
        while x < N {
            let moved = reordered[x];
            if str_eq(original, moved) {
                shifts[i] = x;
            }

            x += 1;
        }

        i += 1;
    }

    todo!()
}

#[cfg(test)]
mod tests {

    use crate::macros::hash_face::{LenPosBuckets, sorted_fields_shifts};

    use super::build_buckets;

    #[test]
    fn ab() {
        let buckets = build_buckets::<2, 9, 6>(&["abcdse", "b23"]);
        panic!("{:#?}", sorted_fields_shifts(buckets));
    }

    #[test]
    fn win_config() {
        const FIELDS: [&str; 22] = [
            "row",
            "col",
            "width",
            "height",
            "anchor",
            "relative",
            "split",
            "win",
            "bufpos",
            "external",
            "focusable",
            "vertical",
            "zindex",
            "border",
            "title",
            "title_pos",
            "footer",
            "footer_pos",
            "style",
            "noautocmd",
            "fixed",
            "hide",
        ];

        const EXPECTED: [&str; 22] = [
            "col",
            "row",
            "win",
            "hide",
            "width",
            "split",
            "title",
            "fixed",
            "style",
            "anchor",
            "bufpos",
            "height",
            "zindex",
            "footer",
            "border",
            "external",
            "relative",
            "vertical",
            "focusable",
            "noautocmd",
            "title_pos",
            "footer_pos",
        ];

        const SUM_LEN: usize = const {
            let mut sum = 0;
            let mut i = 0;
            while i < EXPECTED.len() {
                sum += EXPECTED[i].len();

                i += 1;
            }

            sum
        };

        const MAX_LEN: usize = const {
            let mut max = 0;
            let mut i = 0;
            while i < EXPECTED.len() {
                if max < EXPECTED[i].len() {
                    max = EXPECTED[i].len();
                }

                i += 1;
            }

            max
        };

        const LEN_POS_BUCKETS: LenPosBuckets<22, SUM_LEN, MAX_LEN> = build_buckets(&FIELDS);
        //panic!("{:#?}", LEN_POS_BUCKETS);
        let s = sorted_fields_shifts(LEN_POS_BUCKETS);

        assert_eq!(s, EXPECTED);
    }
}
