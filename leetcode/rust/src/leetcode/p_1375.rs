// 1375. Number of Times Binary String Is Prefix-Aligned
// -----------------------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut values = BinaryHeap::new();
        let mut ans = 0;
        flips.into_iter().for_each(|n| {
            values.push(n);
            let max = *values.peek().unwrap() as usize;
            if max == values.len() {
                ans += 1;
            }
        });

        ans
    }
}
