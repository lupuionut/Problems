// 1980. Find Unique Binary String
// -------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut set = HashSet::new();

        for i in 0..nums.len() {
            set.insert(&nums[i]);
        }

        for i in 0..(1 << n) {
            let bin = format!("{:0n$b}", i);
            if !set.contains(&bin) {
                return bin;
            }
        }
        "".to_string()
    }
}
