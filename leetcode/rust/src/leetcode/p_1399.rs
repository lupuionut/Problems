// 1399. Count Largest Group
// -------------------------

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        let mut counter = HashMap::new();
        let mut ans = 0;

        for i in 1..=n {
            let res = dp[(i / 10) as usize] + i % 10;
            dp[i as usize] = res;
            counter.entry(res).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut mx = 0;
        for &v in counter.values() {
            if v > mx {
                mx = v;
                ans = 1;
            } else if v == mx {
                ans += 1;
            }
        }
        ans
    }
}
