// 2537. Count the Number of Good Subarrays
// ----------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans = 0;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut l = 0;
        let mut curr = 0;

        for r in 0..nums.len() {
            let n = nums[r];
            counter.entry(n).and_modify(|c| *c += 1).or_insert(1);
            if let Some(&v) = counter.get(&n) {
                curr -= ((v - 2) * (v - 1) / 2);
                curr += ((v - 1) * v / 2);
            }

            while curr >= k {
                ans += (nums.len() - r) as i64;
                if let Some(&v) = counter.get(&nums[l]) {
                    curr += ((v - 2) * (v - 1) / 2);
                    curr -= ((v - 1) * v / 2);
                }
                counter.entry(nums[l]).and_modify(|c| *c -= 1);
                l += 1;
            }
        }

        ans
    }
}
