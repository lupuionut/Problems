// 1207. Unique Number of Occurrences
// ----------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occurences = HashMap::new();
        let mut occ_unique = HashSet::new();
        arr.iter().for_each(|&n| {
            occurences.entry(n).and_modify(|c| *c += 1).or_insert(1);
        });
        for (_, val) in occurences.iter() {
            if occ_unique.contains(&val) {
                return false;
            }
            occ_unique.insert(val);
        }

        true
    }
}
