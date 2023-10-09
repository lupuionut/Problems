// 34. Find First and Last Position of Element in Sorted Array
// -----------------------------------------------------------

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![-1; 2];
        let mut left = 0;
        let mut right = (nums.len() - 1) as i32;

        while left <= right {
            let middle = (left + right) / 2;
            if nums[middle as usize] < target {
                left = middle + 1;
            } else if nums[middle as usize] > target {
                right = middle - 1;
            } else {
                left = middle;
                break;
            }
        }

        let found = left;

        // if there is at least one target
        if (left as usize) < nums.len() && nums[left as usize] == target {
            // check the left side of target
            right = left;
            left = 0;

            while left <= right {
                let middle = (left + right) / 2;
                if nums[middle as usize] < target {
                    left = middle + 1;
                } else {
                    ans[0] = middle;
                    right = middle - 1;
                }
            }

            // check the right side of target
            left = found;
            right = (nums.len() - 1) as i32;

            while left <= right {
                let middle = (left + right) / 2;
                if nums[middle as usize] > target {
                    right = middle - 1;
                } else {
                    ans[1] = middle;
                    left = middle + 1;
                }
            }
        }

        ans
    }
}
