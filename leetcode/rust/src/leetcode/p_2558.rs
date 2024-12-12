// 2558. Take Gifts From the Richest Pile
// --------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::new();
        let mut moves = 0;
        gifts.into_iter().for_each(|n| heap.push(n as i64));

        while moves < k {
            let n = heap.pop().unwrap();
            let m = (n as f64).sqrt().floor() as i64;
            heap.push(m);
            moves += 1;
        }
        heap.iter().sum::<i64>()
    }
}
