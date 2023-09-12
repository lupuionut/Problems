// 1647. Minimum Deletions to Make Character Frequencies Unique
// ------------------------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut ans = 0;
        let mut freq = vec![0; 26];
        let mut seen = HashSet::new();

        s.as_bytes().iter().for_each(|&c| {
            let key = c as usize - 97;
            freq[key] += 1;
        });
        freq.sort_by(|a, b| b.cmp(&a));

        freq.iter().for_each(|&char_freq| {
            if char_freq > 0 {
                if seen.contains(&char_freq) {
                    let mut new_char_freq = char_freq;
                    while seen.contains(&new_char_freq) {
                        new_char_freq -= 1;
                    }
                    if new_char_freq > 0 {
                        seen.insert(new_char_freq);
                    }
                    ans += (char_freq - new_char_freq);
                } else {
                    seen.insert(char_freq);
                }
            }
        });

        ans
    }
}
