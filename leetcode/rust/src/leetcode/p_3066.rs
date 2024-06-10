// 3066. Minimum Operations to Exceed Threshold Value II
// -----------------------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut ans = 0;
        nums.iter().for_each(|&n| heap.push(Reverse(n as i64)));

        while heap.len() > 1 {
            let Reverse(a) = heap.pop().unwrap();
            let Reverse(b) = heap.pop().unwrap();

            if a >= k as i64 {
                break;
            }

            let c = a.min(b) * 2 + a.max(b);
            heap.push(Reverse(c));
            ans += 1;
        }

        ans
    }
}
