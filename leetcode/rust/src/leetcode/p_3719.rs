// 3719. Longest Balanced Subarray I
// ---------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for l in 0..nums.len() {
            let mut odd = HashSet::new();
            let mut even = HashSet::new();
            for r in l..nums.len() {
                if nums[r] % 2 == 0 {
                    even.insert(nums[r]);
                } else {
                    odd.insert(nums[r]);
                }
                if even.len() == odd.len() {
                    ans = ans.max((r - l + 1) as i32);
                }
            }
        }

        ans
    }
}
