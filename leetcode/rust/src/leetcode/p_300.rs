// 300. Longest Increasing Subsequence
// -----------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = HashMap::new();

        nums.iter().for_each(|&n| {
            let mut best = 1;
            for (&key, &value) in dp.iter() {
                if n > key {
                    best = best.max(value + 1);
                }
            }
            dp.insert(n, best);
        });

        let mut ans = 0;
        dp.iter().map(|(&_, &v)| v).max().unwrap()
    }
}
