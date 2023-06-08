// 128. Longest Consecutive Sequence
// ---------------------------------
// Use a hashset to keep track of all entries, and
// iterate over the nums. If a num - 1 doesn't exist
// in the set, it can be the start of a new sequence.

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut longest = 0;

        for i in 0..nums.len() {
            seen.insert(nums[i]);
        }

        for i in 0..nums.len() {
            let previous = nums[i] - 1;
            if !seen.contains(&previous) {
                let mut start = nums[i];
                let mut counter = 0;
                while seen.contains(&start) {
                    counter += 1;
                    start += 1;
                }
                if counter > longest {
                    longest = counter;
                }
            }
        }

        longest
    }
}
