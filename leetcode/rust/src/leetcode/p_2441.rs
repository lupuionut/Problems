// 2441. Largest Positive Integer That Exists With Its Negative
// ------------------------------------------------------------

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in 0..nums.len() {
            if nums[i] >= 0 {
                break;
            }
            if Self::contains(&nums, -nums[i]) {
                return -nums[i];
            }
        }

        -1
    }

    fn contains(nums: &Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] == target {
                return true;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        if left as usize >= nums.len() || right < 0 {
            return false;
        }

        nums[left as usize] == target
    }
}
