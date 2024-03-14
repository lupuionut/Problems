// 930. Binary Subarrays With Sum
// ------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut ans = 0;
        let mut prefixes = HashMap::new();
        prefixes.insert(0, 1);
        let mut total = 0;

        for num in nums {
            total += num;
            let needle = total - goal;
            if let Some(val) = prefixes.get(&needle) {
                ans += val;
            }
            prefixes
                .entry(total)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        ans
    }
}
