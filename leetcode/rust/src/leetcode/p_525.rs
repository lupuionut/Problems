// 525. Contiguous Array
// ---------------------

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut seen = HashMap::new();
        seen.insert(0, -1);
        let mut ans = 0;

        for i in 0..nums.len() {
            total = if nums[i] == 1 { total + 1 } else { total - 1 };
            if let Some(k) = seen.get(&total) {
                ans = ans.max(i as i32 - k);
            } else {
                seen.insert(total, i as i32);
            }
        }

        ans
    }
}
