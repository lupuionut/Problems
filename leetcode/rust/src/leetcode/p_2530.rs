// 2530. Maximal Score After Applying K Operations
// -----------------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, mut k: i32) -> i64 {
        let mut heap = BinaryHeap::new();
        let mut ans = 0;

        nums.iter().for_each(|&n| {
            heap.push(n as i64);
        });

        while k > 0 {
            if let Some(num) = heap.pop() {
                ans += num;
                let new = num / 3;
                if num % 3 != 0 {
                    heap.push(new + 1)
                } else {
                    heap.push(new);
                }
            }
            k -= 1;
        }
        ans
    }
}

