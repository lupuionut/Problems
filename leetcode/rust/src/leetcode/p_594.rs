// 594. Longest Harmonious Subsequence
// -----------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let mut q = VecDeque::new();
        q.push_back(nums[0]);

        for i in 1..nums.len() {
            let curr = nums[i];
            while let Some(&min) = q.front() {
                if curr - min > 1 {
                    if q.len() >= 2 {
                        if let Some(&min) = q.front() {
                            if let Some(&max) = q.back() {
                                if max - min == 1 {
                                    ans = ans.max(q.len() as i32);
                                }
                            }
                        }
                    }
                    q.pop_front();
                } else {
                    break;
                }
            }
            q.push_back(curr);
        }

        if q.len() >= 2 {
            let min = q.pop_front().unwrap();
            let max = q.pop_back().unwrap();
            if max - min == 1 {
                ans = ans.max(2 + q.len() as i32);
            }
        }

        ans
    }
}
