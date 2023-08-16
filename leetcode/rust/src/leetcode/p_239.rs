// 239. Sliding Window Maximum
// ---------------------------
// Keep a monotonic decreasing vec,
// the first elem is the idx of the max element in current window

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = VecDeque::new();
        let k = k as usize;

        for idx in 0..nums.len() {
            while stack.len() > 0 && nums[*stack.back().unwrap()] <= nums[idx] {
                stack.pop_back();
            }
            stack.push_back(idx);
            if idx >= k - 1 {
                if let Some(&first) = stack.front() {
                    ans.push(nums[first]);
                    if first == idx - k + 1 {
                        stack.pop_front();
                    }
                }
            }
        }

        ans
    }
}
