// 3105. Longest Strictly Increasing or Strictly Decreasing Subarray
// -----------------------------------------------------------------

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut strike = 1;
        let mut sign = 0;
        let mut last = nums[0];
        let mut ans = 0;

        for i in 1..nums.len() {
            if sign == 0 {
                if nums[i] > last {
                    sign = 1;
                } else if nums[i] < last {
                    sign = -1;
                }
            }
            if (nums[i] > last && sign == 1) || (nums[i] < last && sign == -1) {
                strike += 1;
            } else {
                ans = ans.max(strike);
                strike = if nums[i] > last || nums[i] < last {2} else {1};
                sign *= -1;
            }
            last = nums[i];
        }

        ans = ans.max(strike);
        ans
    }
}
