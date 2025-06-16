// 2016. Maximum Difference Between Increasing Elements
// ----------------------------------------------------

use std::collections::BinaryHeap;
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut h: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut ans = i32::MIN;

        nums.iter().enumerate().for_each(|(k, &v)| {
            h.push((v, k));
        });

        for i in 0..nums.len() - 1 {
            while let Some(&(val, key)) = h.peek() {
                if key <= i {
                    h.pop();
                } else {
                    if nums[i] < val {
                        ans = ans.max(val - nums[i]);
                    }
                    break;
                }
            }
        }

        if ans == i32::MIN {
            return -1;
        }
        ans
    }
}
