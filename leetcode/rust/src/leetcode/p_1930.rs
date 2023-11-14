// 1930. Unique Length-3 Palindromic Subsequences
// ----------------------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut ans = 0;
        // map of start, end for each letter
        let mut letters: HashMap<char, (usize, usize)> = HashMap::new();

        s.chars().enumerate().for_each(|(k, v)| {
            letters
                .entry(v)
                .and_modify(|positions| positions.1 = k)
                .or_insert((k, k));
        });

        for (_, position) in letters.iter() {
            let mut set = HashSet::new();
            for i in position.0 + 1..position.1 {
                set.insert(&s[i..i + 1]);
            }
            ans += set.len();
        }
        ans as i32
    }
}
