// 2948. Make Lexicographically Smallest Array by Swapping Elements
// ----------------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut translate = HashMap::new();
        let mut groups = vec![];
        let mut ans = vec![-1; nums.len()];
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut prev: Option<i32> = None;
        for i in 0..sorted_nums.len() {
            if let Some(x) = prev {
                let n = groups.len() - 1;
                if (sorted_nums[i] - x).abs() > limit {
                    groups.push(vec![sorted_nums[i]]);
                } else {
                    groups[n].push(sorted_nums[i]);
                }
            } else {
                groups.push(vec![sorted_nums[i]]);
            }
            translate.insert(sorted_nums[i], groups.len() - 1);
            prev = Some(sorted_nums[i]);
        }

        groups = groups
            .into_iter()
            .map(|mut g| {
                g.sort_by(|a, b| b.cmp(&a));
                return g;
            })
            .collect();

        for i in 0..nums.len() {
            let k = *translate.get(&nums[i]).unwrap();
            let n = groups[k].pop().unwrap();
            ans[i] = n;
        }

        ans
    }
}
