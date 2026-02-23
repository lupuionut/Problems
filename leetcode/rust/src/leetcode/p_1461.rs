// 1461. Check If a String Contains All Binary Codes of Size K
// -----------------------------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let target = 1 << k;
        let mut seen = HashSet::new();
        let k = k as usize;
        if k > s.len() {
            return false;
        }

        for i in 0..s.len() + 1 - k {
            seen.insert(&s[i..i + k]);
        }

        if seen.len() == target as usize {
            return true;
        }
        false
    }
}
