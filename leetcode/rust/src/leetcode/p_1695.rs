// 1695. Maximum Erasure Value
// ---------------------------

use std::collections::HashSet;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut set = HashSet::new();
        let mut sum = 0;
        let mut ans = 0;
        for r in 0..nums.len() {
            while set.contains(&nums[r]) {
                set.remove(&nums[l]);
                sum -= nums[l];
                l += 1;
            }
            set.insert(nums[r]);
            sum += nums[r];
            ans = ans.max(sum);
        }
        ans
    }
}
