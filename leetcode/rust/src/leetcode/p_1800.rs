// 1800. Maximum Ascending Subarray Sum
// ------------------------------------

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut best = 0;
        let mut prev = -1;
        let mut t = 0;

        for r in 0..nums.len() {
            if nums[r] <= prev {
                l = r;
                prev = -1;
                t = 0;
            }
            prev = nums[r];
            t += nums[r];
            best = best.max(t);
        }

        best
    }
}
