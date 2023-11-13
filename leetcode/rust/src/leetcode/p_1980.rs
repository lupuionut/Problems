// 1980. Find Unique Binary String
// -------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut existing = HashSet::new();
        nums.iter().for_each(|s| {
            existing.insert(s);
        });
        let mut curr = vec!['0'; nums.len()];

        fn recurse(
            idx: usize,
            mx: usize,
            curr: &mut Vec<char>,
            existing: &HashSet<&String>,
        ) -> Option<String> {
            if idx == mx {
                let val = curr.iter().collect::<String>();
                if !existing.contains(&val) {
                    return Some(val);
                } else {
                    return None;
                }
            } else {
                curr[idx] = '0';
                let res = recurse(idx + 1, mx, curr, existing);
                if res.is_some() {
                    return res;
                } else {
                    curr[idx] = '1';
                    let s = recurse(idx + 1, mx, curr, existing);
                    return s;
                }
            }
        }

        recurse(0, nums.len(), &mut curr, &existing).unwrap()
    }
}
