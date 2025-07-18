// 2163. Minimum Difference in Sums After Removal of Elements
// ----------------------------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.iter().map(|&n| n as i64).collect();
        let mut total_right = 0; // min
        let mut total_left = 0; // max
        let mut left_heap: BinaryHeap<i64> = BinaryHeap::new(); // min heap
        let mut right_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new(); // max heap
        let n = nums.len();
        let t = n / 3;
        let mut best_right: Vec<i64> = vec![];
        let mut ans = 1 << 52;

        for i in (t..n).rev() {
            total_right += nums[i];
            right_heap.push(Reverse(nums[i]));
            while right_heap.len() > t {
                if let Some(Reverse(v)) = right_heap.pop() {
                    total_right -= v;
                }
            }
            if right_heap.len() == t {
                best_right.push(total_right);
            }
        }

        let mut j = best_right.len() - 1;
        for i in 0..n - t {
            total_left += nums[i];
            left_heap.push(nums[i]);
            while left_heap.len() > t {
                if let Some(v) = left_heap.pop() {
                    total_left -= v;
                }
            }
            if left_heap.len() == t {
                ans = ans.min(total_left - best_right[j]);
                j -= 1;
            }
        }

        ans
    }
}
