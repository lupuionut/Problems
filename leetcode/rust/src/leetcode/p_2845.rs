// 2845. Count of Interesting Subarrays
// ------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut ans = 0;
        let mut ps: HashMap<i32, i64> = HashMap::new();
        ps.insert(0, 1);

        let mut ans = 0;
        let mut curr = 0;
        for num in nums {
            if num % modulo == k {
                curr += 1;
            }
            let target = (curr - k) % modulo;
            if let Some(&v) = ps.get(&target) {
                ans += v;
            }
            ps.entry((curr % modulo))
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        ans
    }
}
