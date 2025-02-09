// 2364. Count Number of Bad Pairs
// -------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut freq = HashMap::new();

        for i in 0..nums.len() {
            let j = i as i32;
            if let Some(t) = freq.get(&(nums[j as usize] - j)) {
                ans += ((j - t) as i64);
            } else {
                ans += j as i64;
            }

            freq.entry(nums[i] - i as i32)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        ans
    }
}
