// 857. Minimum Cost to Hire K Workers
// -----------------------------------

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut ratios = vec![];
        let mut heap = BinaryHeap::new();
        let mut ans = f64::MAX;
        let mut total = 0f64;

        for i in 0..quality.len() {
            ratios.push((wage[i] as f64 / quality[i] as f64, quality[i]));
        }
        ratios.sort_by(|a, b| {
            if a.0 < b.0 {
                return Ordering::Less;
            } else if a.0 > b.0 {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        });

        for i in 0..ratios.len() {
            total += ratios[i].1 as f64;
            heap.push(ratios[i].1);
            if heap.len() > k as usize {
                total -= heap.pop().unwrap() as f64;
            }
            if heap.len() == k as usize {
                ans = ans.min(ratios[i].0 * total as f64);
            }
        }

        ans
    }
}
