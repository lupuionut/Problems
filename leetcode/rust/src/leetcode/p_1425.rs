// 1425. Constrained Subsequence Sum
// ---------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut window: VecDeque<(usize, i32)> = VecDeque::new();
        let mut best = nums.clone();

        nums.iter().enumerate().for_each(|(i, &v)| {
            if let Some(&biggest) = window.front() {
                if biggest.1 > 0 {
                    best[i] += (biggest.1);
                }
                // eliminate oldest value if window is already at full capacity
                if i - biggest.0 == k as usize {
                    window.pop_front();
                }
            }
            // mantain monotonically decreasing dequeue
            // so biggest will always be window's front
            while let Some(&last) = window.back() {
                if last.1 > best[i] {
                    break;
                } else {
                    window.pop_back();
                }
            }
            window.push_back((i, best[i]));
        });

        *best.iter().max().unwrap()
    }
}
