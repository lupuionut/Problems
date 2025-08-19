// 2348. Number of Zero-Filled Subarrays
// -------------------------------------

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut streak = 0i64;

        for right in 0..nums.len() {
            if nums[right] != 0 {
                ans += (streak * (streak + 1)) / 2;
                streak = 0;
            } else {
                streak += 1;
            }
        }
        ans += (streak * (streak + 1)) / 2;
        ans
    }
}
