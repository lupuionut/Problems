// 1248. Count Number of Nice Subarrays
// ------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut keys = VecDeque::new();

        for right in 0..nums.len() {
            if nums[right] % 2 == 1 {
                keys.push_back(right);
            }
            while keys.len() > k as usize {
                left = keys.pop_front().unwrap() + 1;
            }

            if keys.len() == k as usize {
                let front = *keys.front().unwrap();
                ans += (front - left + 1) as i32;
            }
        }

        ans
    }
}
