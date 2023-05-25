// 2261. K Divisible Elements Subarrays

use std::collections::HashSet;

impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let mut ans = 0;
        let mut visited: HashSet<String> = HashSet::new();

        for i in 0..nums.len() {
            let mut counter = 0;
            let mut hash = "".to_owned();
            let mut current: Vec<i32> = Vec::new();
            for j in i..nums.len() {
                if counter > k {
                    break;
                }
                let n = nums[j];
                hash.push_str("-");
                hash.push_str(&n.to_string());

                if n % p == 0 {
                    counter += 1;
                }
                if counter <= k && !visited.contains(&hash) {
                    ans += 1;
                }
                visited.insert(hash.clone());
            }
        }
        ans
    }
}
