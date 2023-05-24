// 491. Non-decreasing Subsequences
// decision tree, hashset

use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        fn backtrack(
            start: i32,
            nums: &Vec<i32>,
            current: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) -> () {
            let mut seen: HashSet<i32> = HashSet::new();
            if current.len() > 1 {
                ans.push(current.to_vec());
            }

            let last = if current.len() > 0 {
                current[current.len() - 1]
            } else {
                -200
            };

            for i in start..nums.len() as i32 {
                if seen.contains(&nums[i as usize]) {
                    continue;
                }

                if nums[i as usize] >= last {
                    current.push(nums[i as usize]);
                    backtrack(i + 1, nums, current, ans);
                    current.pop();
                }

                seen.insert(nums[i as usize]);
            }
        }

        let mut current: Vec<i32> = Vec::new();
        backtrack(0, &nums, &mut current, &mut ans);

        ans
    }
}
