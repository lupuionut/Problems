// 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
// --------------------------------------------------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut decreasing: VecDeque<(usize, i32)> = VecDeque::new();
        let mut increasing: VecDeque<(usize, i32)> = VecDeque::new();

        for right in 0..nums.len() {
            while decreasing.len() > 0 && nums[right] > decreasing.back().unwrap().1 {
                decreasing.pop_back();
            }
            decreasing.push_back((right, nums[right]));
            while increasing.len() > 0 && nums[right] < increasing.back().unwrap().1 {
                increasing.pop_back();
            }
            increasing.push_back((right, nums[right]));
            while decreasing.front().unwrap().1 - increasing.front().unwrap().1 > limit {
                left += 1;
                while decreasing.front().unwrap().0 < left {
                    decreasing.pop_front();
                }
                while increasing.front().unwrap().0 < left {
                    increasing.pop_front();
                }
            }
            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}
