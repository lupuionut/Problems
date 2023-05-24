// 2542. Maximum Subsequence Score
// -------------------------------
// greedy, maximize, minheap

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut pairs: Vec<(i32, i32)> = Vec::new();
        let mut hp: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for i in 0..nums1.len() {
            pairs.push((nums1[i], nums2[i]));
        }
        pairs.sort_by(|a, b| (b.1).cmp(&a.1));

        let mut sum: i64 = 0;
        let mut score: i64 = 0;

        for pair in pairs {
            sum += pair.0 as i64;
            hp.push(Reverse(pair.0));

            if hp.len() > k as usize {
                if let Some(Reverse(excluded)) = hp.pop() {
                    sum -= excluded as i64
                }
            }

            if hp.len() == k as usize {
                let cur_score = sum as i64 * pair.1 as i64;
                if cur_score > score {
                    score = cur_score;
                }
            }
        }
        score
    }
}
