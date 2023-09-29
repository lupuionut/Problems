// 896. Monotonic Array
// --------------------

use std::cmp::Ordering;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }
        let mut state = None;
        for i in 1..nums.len() {
            let order = nums[i].cmp(&nums[i - 1]);
            if order != Ordering::Equal {
                if let Some(rule) = state {
                    if rule != order {
                        return false;
                    }
                } else {
                    state = Some(order);
                }
            }
        }

        true
    }
}
