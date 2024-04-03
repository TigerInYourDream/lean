pub fn selection_sort(nums: &mut [i32]) {
    let len = nums.len();
    for i in 0..len - 1 {
        let mut min = i;
        for j in i + 1..len {
            if nums[j] < nums[min] {
                min = j;
            }
        }
        nums.swap(i, min);
    }
}

pub fn bubule_sort(nums: &mut [i32]) {
    let len = nums.len();
    for i in (0..len).rev() {
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1)
            }
        }
    }
}

pub fn insert_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let base = nums[i];
        let mut j = (i - 1) as i32;
        while j >= 0 && nums[j as usize] > base {
            nums[(j + 1) as usize] = nums[j as usize];
            j -= 1;
        }
        nums[(j + 1) as usize] = base;
    }
}

pub fn merge_sort(nums: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mid = (left + right) / 2;
    merge_sort(nums, left, mid);
    merge_sort(nums, mid + 1, right);
    merge(nums, left, mid, right);
}

fn merge(nums: &mut [i32], left: usize, mid: usize, right: usize) {
    let temp_size = right - left + 1;
    let mut temp = vec![0; temp_size];

    let mut i = left;
    let mut j = mid + 1;
    let mut k = 0;

    while i <= mid && j <= right {
        if nums[i] <= nums[j] {
            temp[k] = nums[i];
            i += 1;
        } else {
            temp[k] = nums[j];
            j += 1;
        }
        k += 1;
    }

    // poll left in left or nums vec
    while i <= mid {
        temp[k] = nums[i];
        i += 1;
        k += 1;
    }

    while j <= right {
        temp[k] = nums[j];
        j += 1;
        k += 1;
    }

    for i in 0..temp_size {
        nums[left + i] = temp[i as usize];
    }
}

#[cfg(test)]
mod test {
    use crate::sort::{merge2, merge_sort};

    #[test]
    pub fn test_selection_sort() {
        let mut nums = vec![4, 3, 2, 1];
        super::selection_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }

    #[test]
    pub fn test_bubble_sort() {
        let mut nums = vec![4, 3, 2, 1];
        super::bubule_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }

    #[test]
    pub fn test_insert_sort() {
        let mut nums = vec![4, 3, 2, 1];
        super::insert_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }

    #[test]
    pub fn test_merge_sort() {
        let mut nums = [7, 3, 2, 6, 0, 1, 5, 4];
        let right = nums.len() - 1;
        merge_sort(&mut nums, 0, right);
        println!("归并排序完成后 nums = {:?}", nums);
    }

    #[test]
    pub fn test_merge2() {
        let mut nums1 = [1, 3, 9, 11, 12, 15, 16, 17];
        let mut nums2 = [2, 3, 4, 7, 8, 11, 16, 17];
        merge2(&mut nums1, &mut nums2);
    }
}

pub fn merge2(nums1: &mut [i32], nums2: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    let len1 = nums1.len();
    let len2 = nums2.len();

    let mut temp = vec![0; len1 + len2];
    while i < len1 && j < len2 {
        if nums1[i] <= nums2[j] {
            temp[k] = nums1[i];
            i += 1;
        } else {
            temp[k] = nums2[j];
            j += 1;
        }
        k += 1;
    }

    while i < len1 {
        temp[k] = nums1[i];
        i += 1;
        k += 1;
    }

    while j < len2 {
        temp[k] = nums2[j];
        j += 1;
        k += 1;
    }

    println!("k = {}", k);
    println!("sum_len = {}", len1 + len2);
    println!("temp = {:?}", temp);
    println!("temp len = {:?}", temp.len());
}
