// 2779. Maximum Beauty of an Array After Applying Operation
// ---------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut events = HashMap::new();

        for i in 0..nums.len() {
            events
                .entry((nums[i] - k))
                .and_modify(|c| *c += 1)
                .or_insert(1);
            events
                .entry((nums[i] + k + 1))
                .and_modify(|c| *c -= 1)
                .or_insert(-1);
        }

        let mut keys = events.keys().collect::<Vec<_>>();
        keys.sort();
        let mut count = 0;
        for key in keys {
            count += events.get(key).unwrap();
            ans = ans.max(count);
        }

        ans
    }
}
