// 1027. Longest Arithmetic Subsequence
// ------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); nums.len()];
        let mut longest = 1;

        for i in 0..nums.len() {
            let current = nums[i];
            for j in 0..i {
                let diff = nums[i] - nums[j];
                match dp[j].get(&diff) {
                    Some(&v) => {
                        dp[i].insert(diff, v + 1);
                        longest = longest.max(v + 1);
                    }
                    None => {
                        dp[i].insert(diff, 1);
                    }
                };
            }
        }

        longest + 1
    }
}
