pub fn selection_sort(num: &mut [i32]) {
    let len = num.len();
    for i in 0..len - 1 {
        let mut min = i;
        for j in i + 1..len {
            if num[j] < num[min] {
                min = j;
            }
        }
        num.swap(i, min);
    }
}

pub fn bubble_sort(num: &mut [i32]) {
    let len = num.len();
    for i in (0..len).rev() {
        for j in 0..i {
            if num[j] > num[j + 1] {
                num.swap(j, j + 1)
            }
        }
    }
}

pub fn insert_sort(num: &mut [i32]) {
    for i in 1..num.len() {
        let base = num[i];
        let mut j = (i - 1) as i32;
        while j >= 0 && num[j as usize] > base {
            num[(j + 1) as usize] = num[j as usize];
            j -= 1;
        }
        num[(j + 1) as usize] = base;
    }
}

pub fn merge_sort(num: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mid = (left + right) / 2;
    merge_sort(num, left, mid);
    merge_sort(num, mid + 1, right);
    merge(num, left, mid, right);
}

fn merge(num: &mut [i32], left: usize, mid: usize, right: usize) {
    let temp_size = right - left + 1;
    let mut temp = vec![0; temp_size];

    let mut i = left;
    let mut j = mid + 1;
    let mut k = 0;

    while i <= mid && j <= right {
        if num[i] <= num[j] {
            temp[k] = num[i];
            i += 1;
        } else {
            temp[k] = num[j];
            j += 1;
        }
        k += 1;
    }

    // poll left in left or num vec
    while i <= mid {
        temp[k] = num[i];
        i += 1;
        k += 1;
    }

    while j <= right {
        temp[k] = num[j];
        j += 1;
        k += 1;
    }

    num[left..(temp_size + left)].copy_from_slice(&temp[..temp_size]);
}

pub fn merge2(num1: &mut [i32], num2: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    let len1 = num1.len();
    let len2 = num2.len();

    let mut temp = vec![0; len1 + len2];
    while i < len1 && j < len2 {
        if num1[i] <= num2[j] {
            temp[k] = num1[i];
            i += 1;
        } else {
            temp[k] = num2[j];
            j += 1;
        }
        k += 1;
    }

    while i < len1 {
        temp[k] = num1[i];
        i += 1;
        k += 1;
    }

    while j < len2 {
        temp[k] = num2[j];
        j += 1;
        k += 1;

        println!("k = {}", k);
        println!("sum_len = {}", len1 + len2);
        println!("temp = {:?}", temp);
        println!("temp len = {:?}", temp.len());
    }
}

#[cfg(test)]
mod test {
    use crate::sort::{merge2, merge_sort};

    #[test]
    pub fn test_selection_sort() {
        let mut num = vec![4, 3, 2, 1];
        super::selection_sort(&mut num);
        assert_eq!(num, vec![1, 2, 3, 4]);
    }

    #[test]
    pub fn test_bubble_sort() {
        let mut num = vec![4, 3, 2, 1];
        super::bubble_sort(&mut num);
        assert_eq!(num, vec![1, 2, 3, 4]);
    }

    #[test]
    pub fn test_insert_sort() {
        let mut num = vec![4, 3, 2, 1];
        super::insert_sort(&mut num);
        assert_eq!(num, vec![1, 2, 3, 4]);
    }

    #[test]
    pub fn test_merge_sort() {
        let mut num = [7, 3, 2, 6, 0, 1, 5, 4];
        let right = num.len() - 1;
        merge_sort(&mut num, 0, right);
        println!("归并排序完成后 num = {:?}", num);
    }

    #[test]
    pub fn test_merge2() {
        let mut num1 = [1, 3, 9, 11, 12, 15, 16, 17];
        let mut num2 = [2, 3, 4, 7, 8, 11, 16, 17];
        merge2(&mut num1, &mut num2);
    }
}
