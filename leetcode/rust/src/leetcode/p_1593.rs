// 1593. Split a String Into the Max Number of Unique Substrings
// -------------------------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut curr = HashSet::new();
        let chrs = s.chars().collect::<Vec<_>>();
        Self::recurse(0, &chrs, &mut curr)
    }
    pub fn recurse(i: usize, chrs: &Vec<char>, curr: &mut HashSet<String>) -> i32 {
        if i == chrs.len() {
            return 0;
        }
        let mut best = 0;
        for j in i..chrs.len() {
            let s = &chrs[i..j + 1].iter().collect::<String>();
            if curr.contains(s) {
                continue;
            } else {
                curr.insert(s.clone());
                best = best.max(1 + Self::recurse(j + 1, chrs, curr));
                curr.remove(s);
            }
        }

        best
    }
}
