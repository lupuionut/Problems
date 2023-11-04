// 2244. Minimum Rounds to Complete All Tasks
// ------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut freq = HashMap::new();
        tasks.iter().for_each(|x| {
            freq.entry(x)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        let mut ans = 0;
        for &val in freq.values() {
            if val == 1 {
                return -1;
            }
            ans += (val + 2) / 3;
        }

        ans
    }
}
