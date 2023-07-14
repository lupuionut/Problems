// 1218. Longest Arithmetic Subsequence of Given Difference
// --------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut seq: HashMap<i32, i32> = HashMap::new();

        for a in arr {
            let search = a - difference;
            if let Some(v) = seq.get(&search) {
                seq.insert(a, *v + 1);
            } else {
                seq.insert(a, 1);
            }
        }

        seq.into_values().max().unwrap()
    }
}
