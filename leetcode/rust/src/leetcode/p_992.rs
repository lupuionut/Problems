// 992. Subarrays with K Different Integers
// ----------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        fn count(nums: &Vec<i32>, k: i32) -> i32 {
            let mut ans = 0;
            let mut left = 0;
            let mut freqs = HashMap::new();

            for right in 0..nums.len() {
                freqs
                    .entry(nums[right])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                while freqs.len() > k as usize {
                    freqs.entry(nums[left]).and_modify(|c| *c -= 1);
                    if let Some(&val) = freqs.get(&nums[left]) {
                        if val == 0 {
                            freqs.remove_entry(&nums[left]);
                        }
                    }
                    left += 1;
                }
                ans += (right - left + 1) as i32;
            }

            ans
        }
        count(&nums, k) - count(&nums, k - 1)
    }
}
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        fn count(nums: &Vec<i32>, k: i32) -> i32 {
            let mut ans = 0;
            let mut left = 0;
            let mut freqs = HashMap::new();

            for right in 0..nums.len() {
                freqs
                    .entry(nums[right])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                while freqs.len() > k as usize {
                    freqs.entry(nums[left]).and_modify(|c| *c -= 1);
                    if let Some(&val) = freqs.get(&nums[left]) {
                        if val == 0 {
                            freqs.remove_entry(&nums[left]);
                        }
                    }
                    left += 1;
                }
                ans += (right - left + 1) as i32;
            }

            ans
        }
        count(&nums, k) - count(&nums, k - 1)
    }
}
