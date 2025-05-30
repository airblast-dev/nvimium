#[derive(Clone, Copy)]
struct KeyValue<const N: usize> {
    key: usize,
    len: usize,
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
        self.val[self.len] = s;
        self.len += 1;
    }
}

#[derive(Clone, Copy)]
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
                self.kvs[self.find_kv_idx(0).unwrap()].append(s);
                self.len += 1;
            }
        }
    }

    const fn find_kv_idx(&self, key: usize) -> Option<usize> {
        let mut i = 0;
        while i < self.len {
            if self.kvs[i].key == key {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    const fn keys(&self) -> [usize; SUM_LEN] {
        let mut keys = [0; SUM_LEN];
        let mut i = 0;
        while i < self.len {
            keys[i] = self.kvs[i].key;

            i += 1;
        }

        keys
    }
}

#[derive(Clone, Copy)]
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
const fn build_buckets<const N: usize, const SUM_LEN: usize>(
    strings: &[&'static str; N],
) -> (LenPosBuckets<SUM_LEN, SUM_LEN>, usize, usize) {
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
    let mut worst_buck_size = 0;

    let mut len = 0;
    let iter_limit = if max_len > len_buckets.len {
        len_buckets.len
    } else {
        max_len
    };
    while len < iter_limit {
        let strs = len_buckets.kvs[len];

        let mut best_pos = 0;
        let mut min_size = strs.len * 2;
        let mut best_bucket = Bucket::empty();

        let mut pos = 0;
        while pos < len {
            let mut try_bucket = Bucket::<SUM_LEN, SUM_LEN>::empty();
            let mut stri = 0;
            while stri < strs.len {
                let s = strs.val[stri];
                let pos_char = s.as_bytes()[pos] as usize;
                try_bucket.append(pos_char, s);

                stri += 1;
            }

            let mut max_size = 1;
            let mut pos_strs_i = 0;
            while pos_strs_i < try_bucket.len {
                let pos_strs = try_bucket.kvs[pos_strs_i];
                if max_size < try_bucket.len {
                    max_size = pos_strs.len;
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

        len_pos_buckets.len_pos[len] = LenPos {
            pos: best_pos,
            bucket: best_bucket,
        };

        if worst_buck_size < min_size {
            worst_buck_size = min_size
        }

        len += 1;
    }

    (len_pos_buckets, max_len, worst_buck_size)
}

const fn reorder<const N: usize, const SUM_LEN: usize>(
    len_pos_buckets: LenPosBuckets<SUM_LEN, SUM_LEN>,
    max_len: usize,
) -> [&'static str; N] {
    let mut new_order = [""; N];
    let mut new_order_len = 0;
    let iter_count = if len_pos_buckets.len > max_len {
        max_len
    } else {
        len_pos_buckets.len
    };

    let mut len = 0;
    while len < iter_count {
        let vals = &len_pos_buckets.len_pos[len];
        let pos_buck = &vals.bucket;
        let mut keys = pos_buck.keys();
        if pos_buck.len > 1 {
            sort_ints(&mut keys);
            let mut i = 0;
            while i < pos_buck.len {
                let c = keys[i];
                let buck = &pos_buck.kvs[pos_buck.find_kv_idx(c).unwrap()];
                extend_arr(&mut new_order_len, &mut new_order, buck.len, &buck.val);

                i += 1;
            }
        } else {
            new_order[new_order_len] = pos_buck.kvs[pos_buck.find_kv_idx(keys[0]).unwrap()].val[0];
            new_order_len += 1;
        }
        len += 1;
    }

    new_order
}

const fn strs_to_indexes<const N: usize>(orig: &[&'static str; N], ordered: [&'static str; N]) -> [usize; N]{
    [0; N]
}
