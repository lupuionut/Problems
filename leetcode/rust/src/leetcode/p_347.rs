// 347. Top K Frequent Elements
// ----------------------------
// Create a map of freq tuple (num, frequency).
// Iterate over the map and for each frequency insert the num into the corresponding group.
// Iterate the group vector from end to start and retain only k elements.

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut freq_map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut freq_group: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
        let mut counter = k;

        for v in nums {
            freq_map
                .entry(v)
                .and_modify(|entry| {
                    entry.1 += 1;
                })
                .or_insert((v, 1));
        }
        for (k, v) in freq_map {
            freq_group[v.1 as usize].push(v.0);
        }

        for freq in (0..freq_group.len()).rev() {
            if !freq_group[freq].is_empty() {
                for num in &freq_group[freq] {
                    if counter > 0 {
                        ans.push(*num);
                        counter -= 1;
                    }
                }
            }
        }

        ans
    }
}
