// 974. Subarray Sums Divisible by K
// ---------------------------------
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut running_sum = 0;
        let mut remainders = HashMap::new();
        remainders.insert(0, 1);
        let mut ans = 0;

        nums.iter().for_each(|&n| {
            running_sum += n;
            let remainder = running_sum.rem_euclid(k);
            if let Some(val) = remainders.get(&remainder) {
                ans += val;
            }
            remainders
                .entry(remainder)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        });

        ans
    }
}
