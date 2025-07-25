// 3487. Maximum Unique Subarray Sum After Deletion
// ------------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut negative = i32::MIN;
        let mut uniq = HashSet::new();
        for num in nums {
            if num < 0 {
                negative = negative.max(num);
            } else {
                uniq.insert(num);
            }
        }
        if uniq.len() > 0 {
            uniq.iter().sum()
        } else {
            negative
        }
    }
}
