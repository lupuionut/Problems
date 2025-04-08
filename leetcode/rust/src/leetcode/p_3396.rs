// 3396. Minimum Number of Operations to Make Elements in Array Distinct
// ---------------------------------------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut last: HashMap<i32, i32> = HashMap::new();
        let mut cut = -1;
        let mut ans = 0;
        for i in 0..nums.len() {
            let val = nums[i];
            if let Some(&pos) = last.get(&val) {
                cut = cut.max(pos);
            }
            last.insert(val, i as i32);
        }

        if (cut + 1) % 3 != 0 {
            (cut + 1) / 3 + 1
        } else {
            (cut + 1) / 3
        }
    }
}
