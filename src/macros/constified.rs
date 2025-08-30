pub(super) const fn sort_ints<const N: usize>(arr: &mut [u8; N], len: usize) {
    loop {
        let mut i = 1;
        let mut swapped = false;
        while i < len {
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

pub(super) const fn extend_arr<T: Copy>(
    len: &mut usize,
    arr: &mut [T],
    ext_len: usize,
    ext_arr: &[T],
) {
    debug_assert!(arr.len() - *len >= ext_len);
    unsafe {
        arr.as_mut_ptr()
            .add(*len)
            .copy_from_nonoverlapping(ext_arr.as_ptr(), ext_len);
    }

    *len += ext_len;
}

pub(crate) const fn strings_len_max(strings: &[&'static str]) -> usize {
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

pub(crate) const fn str_eq(s1: &'static str, s2: &'static str) -> bool {
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

pub(crate) const fn count_unique_chars(strings: &[&str]) -> usize {
    let mut counter = 0_u128;
    let mut i = 0;
    let ss_len = strings.len();
    while i < ss_len {
        let mut ci = 0;
        let string = strings[i];
        let s_len = string.len();
        while ci < s_len {
            let byte = string.as_bytes()[ci];
            counter |= 1 << byte as u128;

            ci += 1;
        }

        i += 1;
    }

    counter.count_ones() as usize
}

#[cfg(test)]
mod tests {
    use crate::macros::constified::str_eq;

    #[test]
    fn strings_len_max() {
        assert_eq!(super::strings_len_max(["1", "22", "333"].as_slice()), 3);
        assert_eq!(super::strings_len_max(["111", "22", "3"].as_slice()), 3);
        assert_eq!(super::strings_len_max(["1", "222", "33"].as_slice()), 3);
    }

    #[test]
    fn string_equality() {
        assert!(!str_eq("Hello", "Bye"));
        assert!(!str_eq("123", "234"));
        assert!(!str_eq("123", "23456"));
        assert!(!str_eq("aaaa", "aaaaa"));

        assert!(str_eq("asdasd", "asdasd"));
        assert!(str_eq("AsDDsd", "AsDDsd"));
    }

    #[test]
    fn extend_arr() {
        let mut arr = [0_usize; 20];
        let mut arr_len = 0;
        super::extend_arr(&mut arr_len, &mut arr, 3, &[3; 3]);
        super::extend_arr(&mut arr_len, &mut arr, 4, &[4; 4]);
        super::extend_arr(&mut arr_len, &mut arr, 2, &[5; 2]);
        assert_eq!(
            arr,
            [3, 3, 3, 4, 4, 4, 4, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn sort_ints() {
        let mut to_sort = [5, 4, 3, 2, 1];
        super::sort_ints(&mut to_sort, 5);
        assert_eq!(to_sort, [1, 2, 3, 4, 5]);

        let mut to_sort = [1, 2, 3, 4, 5];
        super::sort_ints(&mut to_sort, 5);
        assert_eq!(to_sort, [1, 2, 3, 4, 5]);
    }
}
