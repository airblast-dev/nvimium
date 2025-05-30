#[derive(Clone, Copy, Debug)]
struct KeyValue<const N: usize> {
    val: [&'static str; N],
    key: usize,
    len: usize,
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
#[repr(Rust)]
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
                new.key = key as u16;
                new.append(s);
                self.len += 1;
            }
        }
    }

    const fn find_kv_idx(&self, key: usize) -> Option<usize> {
        let mut i = 0;
        while i < self.len {
            if (self.kvs[i].key as usize) == key {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    const fn next(&mut self) -> &mut KeyValue<N> {
        &mut self.kvs[self.len]
    }

    const fn keys(&self) -> [u16; SUM_LEN] {
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
struct LenPosBuckets<const N: usize, const SUM_LEN: usize> {
    len: usize,
    len_pos: [LenPos<N, SUM_LEN>; SUM_LEN],
}

impl<const N: usize, const SUM_LEN: usize> LenPosBuckets<N, SUM_LEN> {
    const fn empty() -> Self {
        Self {
            len: 0,
            len_pos: [LenPos::empty(); SUM_LEN],
        }
    }

    const fn find_pos_bucket(&mut self, pos: usize) -> Option<&mut LenPos<N, SUM_LEN>> {
        let mut i = 0;
        while i < SUM_LEN {
            if self.len_pos[i].pos == pos {
                return Some(&mut self.len_pos[i]);
            }

            i += 1;
        }

        None
    }
}

const fn sort_ints(arr: &mut [usize]) {
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

const fn extend_arr<T: Copy>(len: &mut usize, arr: &mut [T], ext_len: usize, ext_arr: &[T]) {
    let mut i = 0;
    while i < ext_len {
        arr[*len] = ext_arr[i];
        *len += 1;

        i += 1;
    }
}

/// A straight forward implementation of the `build_pos_hash` function.
///
/// Intended to be used in const context as it can use a gigantic amount of stack space.
///
/// Various methods are implemented to make this easier to read.
/// https://github.com/neovim/neovim/blob/6c4ddf607f0b0b4b72c4a949d796853aa77db08f/src/gen/hashy.lua#L15C1-L15C35
fn build_buckets<const N: usize, const SUM_LEN: usize>(
    strings: &[&'static str; N],
) -> (LenPosBuckets<SUM_LEN, SUM_LEN>, usize) {
    let mut len_buckets: Bucket<N, N> = Bucket::empty();
    let mut max_len = 0;
    let mut i = 0;

    while i < N {
        let s = strings[i];
        len_buckets.append(s.len(), s);
        if max_len < s.len() {
            max_len = s.len();
        }
        i += 1;
    }

    let mut len_pos_buckets = LenPosBuckets::<SUM_LEN, SUM_LEN>::empty();

    let mut len = 1;
    while len <= max_len {
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

    (len_pos_buckets, max_len)
}

#[cfg(test)]
mod tests {

    use super::build_buckets;

    #[test]
    fn ab() {
        const MAX_UNQIUE_CHAR_COUNT: usize = 60;
        let buckets = build_buckets::<2, 60>(&["abcdse", "b23"]);
        panic!("{:#?}", buckets);
    }
}
