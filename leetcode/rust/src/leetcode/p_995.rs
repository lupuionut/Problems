// 995. Minimum Number of K Consecutive Bit Flips

use std::collections::VecDeque;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let mut flips = VecDeque::new();
        let mut ans = 0;

        for i in 0..nums.len() {
            if flips.len() > 0 && *flips.front().unwrap() < i {
                flips.pop_front();
            }
            let curr = (nums[i] + flips.len() as i32) % 2;
            if curr == 0 {
                if i + k as usize > nums.len() {
                    return -1;
                }
                ans += 1;
                flips.push_back(i + k as usize - 1);
            }
        }
        ans
    }
}
