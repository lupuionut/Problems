// 90. Subsets II
// --------------

use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn make_subset(
            nums: &Vec<i32>,
            curr: &mut Vec<i32>,
            i: usize,
            ans: &mut Vec<Vec<i32>>,
            existing: &mut HashSet<Vec<i32>>,
        ) {
            if i == nums.len() {
                return;
            }
            curr.push(nums[i]);
            if !existing.contains(curr) {
                existing.insert(curr.clone());
                ans.push(curr.clone());
            }
            make_subset(nums, curr, i + 1, ans, existing);
            curr.pop();
            make_subset(nums, curr, i + 1, ans, existing);
        }

        let mut existing = HashSet::new();
        let mut ans = vec![];
        let mut curr = vec![];
        nums.sort();
        ans.push(vec![]);
        make_subset(&nums, &mut curr, 0, &mut ans, &mut existing);
        ans
    }
}
