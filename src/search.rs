 pub fn binary_search(num: &[i32], target: i32) -> isize {
    let mut i = 0;
    let mut j = num.len() - 1;
    while i <= j {
        let mid = i + (j - i) / 2;
        if num[mid] == target {
            return mid as isize;
        } else if num[mid] < target {
            i = mid + 1;
        } else {
            j = mid - 1;
        }
    }
    -1
 }