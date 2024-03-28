// 2958. Length of Longest Subarray With at Most K Frequency
// ---------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freqs = HashMap::new();
        let mut left = 0;
        let mut ans = 0;

        for right in 0..nums.len() {
            let n = nums[right];
            freqs.entry(n).and_modify(|c| *c += 1).or_insert(1);
            let mut val = *freqs.get(&n).unwrap();
            while val > k {
                let m = nums[left];
                freqs.entry(m).and_modify(|c| *c -= 1);
                left += 1;
                if n == m {
                    val -= 1;
                }
            }
            let delta = right - left + 1;
            ans = ans.max(delta as i32);
        }

        ans
    }
}
