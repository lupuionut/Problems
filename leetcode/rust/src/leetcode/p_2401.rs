// 2401. Longest Nice Subarray
// ---------------------------

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let mut acc = 0;

        for r in 0..nums.len() {
            while l < r && (acc & nums[r] != 0) {
                acc ^= nums[l];
                l += 1;
            }
            acc |= nums[r];
            ans = ans.max((r - l + 1) as i32);
        }

        ans
    }
}
