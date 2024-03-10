use std::collections::HashMap;

pub fn binary_search_insertion(num: &[i32], target: i32) -> usize {
    let mut i = 0;
    let mut j = num.len() - 1;
    while i <= j {
        let mid = i + (j - i) / 2;
        if num[mid] < target {
            i = mid + 1;
        } else {
            j = mid - 1;
        }
    }

    i
}

pub fn two_sum_search(num: &[i32], target: i32) -> Option<Vec<i32>> {
    let mut map = HashMap::new();
    for (i, &num) in num.iter().enumerate() {
        let complement = target - num;
        match map.get(&complement) {
            Some(index) => {
                return Some(vec![i as i32, *index as i32]);
            }
            None => {
                map.insert(num, i);
            }
        }
    }
    None
}
