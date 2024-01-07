// 446. Arithmetic Slices II - Subsequence
// ---------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut dp: HashMap<(usize, i64), i32> = HashMap::new();
        for i in 0..nums.len() {
            for j in 0..i {
                let delta: i64 = nums[i] as i64 - nums[j] as i64;
                let val = if let Some(&val) = dp.get(&(j, delta)) {
                    val
                } else {
                    0
                };
                if val != 0 {
                    ans += val;
                }

                dp.entry((i, delta))
                    .and_modify(|count| *count += (val + 1))
                    .or_insert(val + 1);
            }
        }

        ans as i32
    }
}
