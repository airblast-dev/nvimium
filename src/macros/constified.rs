pub(super) const fn sort_ints(arr: &mut [u8]) {
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

pub(super) const fn extend_arr<T: Copy>(
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

pub(crate) const fn strings_len_sum(strings: &[&'static str]) -> usize {
    let mut i = 0;
    let mut sum = 0;
    while i < strings.len() {
        let s = strings[i];

        sum += s.len();

        i += 1;
    }

    sum
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
    fn strings_len_sum() {
        assert_eq!(
            super::strings_len_sum(&["asdasdasd", "vxcv", "123dfsd"]),
            20
        );
        assert_eq!(super::strings_len_sum(&["as", "vxcv", "123dfsd"]), 13);
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
        super::extend_arr(&mut arr_len, &mut arr, 3, &[4; 4]);
        super::extend_arr(&mut arr_len, &mut arr, 3, &[5; 2]);
        assert_eq!(
            arr,
            [3, 3, 3, 4, 4, 4, 4, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn sort_ints() {
        let mut to_sort = [5, 4, 3, 2, 1];
        super::sort_ints(&mut to_sort);
        assert_eq!(to_sort, [1, 2, 3, 4, 5]);

        let mut to_sort = [1, 2, 3, 4, 5];
        super::sort_ints(&mut to_sort);
        assert_eq!(to_sort, [1, 2, 3, 4, 5]);
    }
}
