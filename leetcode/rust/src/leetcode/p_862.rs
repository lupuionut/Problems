// 862. Shortest Subarray with Sum at Least K
// ------------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0i64;
        let mut ans = i64::MAX;
        let mut deque: VecDeque<(i64, usize)> = VecDeque::new();

        for i in 0..nums.len() {
            sum += nums[i] as i64;
            if sum >= k as i64 {
                ans = ans.min((i + 1) as i64);
            }

            while deque.len() > 0 {
                if let Some(&(ps, idx)) = deque.front() {
                    if sum - ps >= k as i64 {
                        ans = ans.min((i - idx) as i64);
                        deque.pop_front();
                    } else {
                        break;
                    }
                }
            }

            while deque.len() > 0 {
                if let Some(&(ps, idx)) = deque.back() {
                    if ps > sum {
                        deque.pop_back();
                    } else {
                        break;
                    }
                }
            }
            deque.push_back((sum, i));
        }

        if ans == i64::MAX {
            -1
        } else {
            ans as i32
        }
    }
}
