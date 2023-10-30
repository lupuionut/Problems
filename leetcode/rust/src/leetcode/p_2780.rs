// 2780. Minimum Index of a Valid Split
// ------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut values = HashMap::new();
        let mut ans = -1;

        nums.iter().for_each(|n| {
            values.entry(n).and_modify(|v| *v += 1).or_insert(1);
        });
        let mut dominant_val = 0;
        let mut dominant_freq = 0;

        for (&key, &val) in values.iter() {
            if val > dominant_freq {
                dominant_val = *key;
                dominant_freq = val;
            }
        }

        let mut appears = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] == dominant_val {
                appears += 1;
                let left = (i + 1) as i32;
                let right = (nums.len() - (i + 1)) as i32;

                if left < 2 * appears && right < 2 * (dominant_freq - appears) {
                    ans = i as i32;
                    break;
                }
            }
        }

        ans
    }
}
