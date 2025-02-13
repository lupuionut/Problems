// 3066. Minimum Operations to Exceed Threshold Value II
// -----------------------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut h: BinaryHeap<i64> =
            BinaryHeap::from(nums.iter().map(|n| -n as i64).collect::<Vec<i64>>());
        let mut ans = 0;
        while h.len() >= 2 {
            let a = -h.pop().unwrap();
            let b = -h.pop().unwrap();
            if a >= k as i64 {
                break;
            }
            ans += 1;
            let c = (a.min(b) * 2) + a.max(b);
            h.push(-c);
        }
        ans
    }
}
