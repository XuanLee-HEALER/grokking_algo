mod binary_search {
    use std::cmp::Ordering;

    pub fn bsearch(my_list: &Vec<i32>, v: &i32) -> Option<usize> {
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::binary_search;

    #[test]
    fn test_bsearch() {
        let list = vec![1, 4, 5, 7, 8, 9, 12, 20];
        let cases = [1, 4, 6, 9, 19, 20];
        let result = [Some(0), Some(1), None, Some(5), None, Some(7)];

        for (i, v) in cases.iter().enumerate() {
            assert_eq!(binary_search::bsearch(&list, v), result[i]);
        }
    }
}
