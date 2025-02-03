// 3105. Longest Strictly Increasing or Strictly Decreasing Subarray
// -----------------------------------------------------------------

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut best = 0;
        let mut prev = nums[0];
        for r in 0..nums.len() {
            if nums[r] <= prev {
                l = r;
            }
            prev = nums[r];
            best = best.max((r - l + 1) as i32);
        }

        l = 0;
        prev = nums[0];
        for r in 0..nums.len() {
            if nums[r] >= prev {
                l = r;
            }
            prev = nums[r];
            best = best.max((r - l + 1) as i32);
        }

        best
    }
}
