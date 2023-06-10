// 2501. Longest Square Streak in an Array
// ---------------------------------------

use std::collections::HashSet;
impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut available: HashSet<u64> = HashSet::new();
        let mut used: HashSet<u64> = HashSet::new();
        let mut longest = 0;
        let mut nums = nums;
        nums.sort();

        for num in &nums {
            available.insert(*num as u64);
        }

        for num in nums {
            if !used.contains(&(num as u64)) {
                let mut n: u64 = num as u64;
                let mut t = 1;
                while available.contains(&(n * n)) {
                    n = n * n;
                    t += 1;
                    used.insert(n);
                }
                if longest < t {
                    longest = t;
                }
            }
        }

        if longest < 2 {
            -1
        } else {
            longest
        }
    }
}
