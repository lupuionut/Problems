// 523. Continuous Subarray Sum
// ----------------------------

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut remainders: HashMap<i32, i32> = HashMap::new();
        remainders.insert(0, -1);
        let mut total = 0;
        for i in 0..nums.len() {
            total += nums[i];
            let remainder = total % k;
            if let Some(position) = remainders.get(&remainder) {
                if i as i32 - position >= 2 {
                    return true;
                }
            } else {
                remainders.insert(remainder, i as i32);
            }
        }
        false
    }
}
