// 3170. Lexicographically Minimum String After Removing Stars
// -----------------------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut valid = vec![vec![]; 26];
        let mut invalid = HashSet::new();
        let mut ans: Vec<char> = vec![];

        for i in 0..s.len() {
            let key = s[i] as usize - 97;
            if s[i] == '*' {
                invalid.insert(i);
                for j in 0..26 {
                    if let Some(n) = valid[j].pop() {
                        invalid.insert(n);
                        break;
                    }
                }
            } else {
                valid[key].push(i);
            }
        }

        let mut j = 0;
        if invalid.len() == 0 {
            ans = s.clone();
        } else {
            for i in 0..s.len() {
                if !invalid.contains(&i) {
                    ans.push(s[i]);
                }
            }
        }
        ans.iter().collect()
    }
}
