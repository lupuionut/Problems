// 1424. Diagonal Traverse II
// --------------------------

use std::collections::HashMap;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut diags: HashMap<usize, Vec<i32>> = HashMap::new();
        let mut ans = vec![];
        let mut mx = 0;

        for i in (0..nums.len()).rev() {
            for j in 0..nums[i].len() {
                let key = i + j;
                mx = mx.max(key);
                diags
                    .entry(key)
                    .and_modify(|values| values.push(nums[i][j]))
                    .or_insert(vec![nums[i][j]]);
            }
        }

        for i in 0..=mx {
            diags.get(&i).unwrap().iter().for_each(|&n| ans.push(n));
        }

        ans
    }
}
