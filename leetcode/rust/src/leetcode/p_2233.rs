// 2233. Maximum Product After K Increments
// ----------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        nums.iter().for_each(|n| {
            heap.push(-n);
        });

        while k > 0 {
            let mut t = heap.pop().unwrap();
            t -= 1;
            heap.push(t);
            k -= 1;
        }

        let mut ans: i64 = 1;
        for n in heap.iter() {
            ans *= (-n) as i64;
            ans %= 1_000_000_007;
        }

        (ans % 1_000_000_007) as i32
    }
}
