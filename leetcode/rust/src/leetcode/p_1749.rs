// 1749. Maximum Absolute Sum of Any Subarray
// ------------------------------------------

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut curr_sum = 0;
        let mut min_prefix_sum = 0;
        let mut max_prefix_sum = 0;
        let mut best = 0;

        for i in 0..nums.len() {
            curr_sum += nums[i];
            best = best.max((curr_sum - min_prefix_sum).abs());
            best = best.max((curr_sum - max_prefix_sum).abs());
            min_prefix_sum = min_prefix_sum.min(curr_sum);
            max_prefix_sum = max_prefix_sum.max(curr_sum);
        }

        best
    }
}
