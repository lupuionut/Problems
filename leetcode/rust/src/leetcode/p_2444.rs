// 2444. Count Subarrays With Fixed Bounds
// ---------------------------------------

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut ans = 0;
        let mut left = 0;
        let mut last_min = -1;
        let mut last_max = -1;

        for right in 0..nums.len() {
            if nums[right] > max_k || nums[right] < min_k {
                left = (right + 1) as i32;
            } else {
                if nums[right] == min_k {
                    last_min = right as i32;
                }
                if nums[right] == max_k {
                    last_max = right as i32;
                }
                if last_min >= left && last_max >= left {
                    ans += (last_max.min(last_min) - left + 1) as i64;
                }
            }
        }
        ans
    }
}
