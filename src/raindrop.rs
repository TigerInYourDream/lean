struct Rain;
impl Rain {
    pub fn sole(height: &[usize]) -> usize {
        let mut res = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut left_max = 0;
        let mut right_max = 0;
        while left < right {
            left_max = std::cmp::max(left_max, height[left]);
            right_max = std::cmp::max(right_max, height[right]);

            if left_max < right_max {
                res += left_max - height[left];
                left += 1;
            } else {
                res += right_max - height[right];
                right -= 1;
            }
        }

        res
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            area = std::cmp::max(area, area_func(left, right, &height));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        area
    }
}

fn area_func(left: usize, right: usize, height: &[i32]) -> i32 {
    let h = std::cmp::min(height[left], height[right]);
    h * (right - left) as i32
}
