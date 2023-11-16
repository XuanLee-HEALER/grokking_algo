mod binary_search {
    use std::cmp::Ordering;

    pub fn bsearch(my_list: &[i32], v: &i32) -> Option<usize> {
        let mut low = 0;
        let mut high = my_list.len() - 1;
        loop {
            if high >= low {
                let mid = (low + high) / 2;
                match my_list[mid].cmp(v) {
                    Ordering::Equal => {
                        return Some(mid);
                    }
                    Ordering::Greater => high = mid - 1,
                    Ordering::Less => low = mid + 1,
                };
            } else {
                return None;
            }
        }
    }
}

mod my_sort {
    use std::cmp::Ordering;

    pub fn selection_sort(my_list: &mut [i32]) {
        let mut idx: usize = 0;
        while idx < my_list.len() - 1 {
            let cur_idx = idx;
            let mut nxt_idx = cur_idx + 1;
            let mut cmp_val = my_list[cur_idx];
            let mut swap_idx = cur_idx;
            while nxt_idx < my_list.len() {
                match cmp_val.cmp(&my_list[nxt_idx]) {
                    Ordering::Equal | Ordering::Greater => {
                        cmp_val = my_list[nxt_idx];
                        swap_idx = nxt_idx;
                    }
                    Ordering::Less => (),
                }
                nxt_idx += 1;
            }
            my_list.swap(cur_idx, swap_idx);

            idx += 1;
        }
    }
}

mod recursion {

    pub fn cal_len<T>(my_list: &[T]) -> u32 {
        if my_list.get(0).is_some() {
            1 + cal_len(&my_list[1..])
        } else {
            0
        }
    }

    pub fn max(my_list: &[i32]) -> Option<i32> {
        if let Some(&ci) = my_list.first() {
            let tr = max(&my_list[1..]);
            if let Some(v) = tr {
                if v > ci {
                    Some(v)
                } else {
                    Some(ci)
                }
            } else {
                Some(ci)
            }
        } else {
            None
        }
    }

    pub fn quick_sort(my_list: &[i32]) -> Vec<i32> {
        if my_list.len() < 2 {
            Vec::from(my_list)
        } else {
            let mut lvec = Vec::new();
            let mut rvec = Vec::new();
            let pivot = 0;
            let mut idx = pivot + 1;
            while idx < my_list.len() {
                let cur_val = my_list[idx];
                if cur_val < my_list[pivot] {
                    lvec.push(cur_val);
                } else {
                    rvec.push(cur_val);
                }
                idx += 1;
            }

            let mut lv = quick_sort(&lvec);
            lv.push(my_list[pivot]);
            lv.append(&mut quick_sort(&rvec));
            lv
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{binary_search, my_sort, recursion};

    #[test]
    fn test_bsearch() {
        let list = vec![1, 4, 5, 7, 8, 9, 12, 20];
        let cases = [1, 4, 6, 9, 19, 20];
        let result = [Some(0), Some(1), None, Some(5), None, Some(7)];

        for (i, v) in cases.iter().enumerate() {
            assert_eq!(binary_search::bsearch(&list, v), result[i]);
        }
    }

    #[test]
    fn test_selection_sort() {
        let mut case = vec![3, 2, 66, 3, 2, 11, 26, 43, 33, 202, 0];
        my_sort::selection_sort(&mut case);
        assert_eq!(case, vec![0, 2, 2, 3, 3, 11, 26, 33, 43, 66, 202]);
        for v in case.iter() {
            print!("{v} ");
        }
        println!()
    }

    #[test]
    fn test_cal_len() {
        let list = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(recursion::cal_len(&list), 7);
    }

    #[test]
    fn test_max() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 6, 10];
        let test_val = recursion::max(&list).unwrap();
        assert_eq!(test_val, 10);
        assert_ne!(test_val, 1);
    }

    #[test]
    fn test_quick_sort() {
        let case = vec![3, 2, 66, 3, 2, 11, 26, 43, 33, 202, 0];
        let result = recursion::quick_sort(&case);
        assert_eq!(result, vec![0, 2, 2, 3, 3, 11, 26, 33, 43, 66, 202]);
        for v in result.iter() {
            print!("{v} ");
        }
        println!()
    }
}
