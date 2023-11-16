// 2530. Maximal Score After Applying K Operations
// -----------------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut ans: i64 = 0;

        fn ceil(n: i32, d: i32) -> i32 {
            let mut r = n / d;
            if n % d != 0 {
                r += 1;
            }
            r
        }

        nums.iter().for_each(|&n| {
            heap.push(n);
        });

        let mut i = 0;

        while i < k {
            let n = heap.pop().unwrap();
            ans += n as i64;
            heap.push(ceil(n, 3));
            i += 1;
        }

        ans
    }
}
