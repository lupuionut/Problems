// 2913. Subarrays Distinct Element Sum of Squares I
// -------------------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut set = HashSet::new();
            for j in i..nums.len() {
                set.insert(nums[j]);
                ans += (set.len() as i32) * (set.len() as i32);
            }
        }
        ans
    }
}
