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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{binary_search, my_sort};

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
}
