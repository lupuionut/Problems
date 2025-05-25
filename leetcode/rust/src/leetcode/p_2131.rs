// 2131. Longest Palindrome by Concatenating Two Letter Words
// ----------------------------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut w: HashMap<String, i32> = HashMap::new();
        let mut ans = 0;
        let mut center = false;
        for word in &words {
            w.entry(word.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        for word in &words {
            let reversed = word.chars().rev().collect::<String>();
            if &reversed == word {
                if let Some(c) = w.get(word) {
                    if c % 2 == 1 {
                        if center == false {
                            center = true;
                            ans += (2 * c);
                        } else {
                            ans += (2 * (c - 1));
                        }
                    } else {
                        ans += (2 * c);
                    }
                }
                w.remove(word);
                continue;
            }

            let mut count = 0;
            if let Some(f) = w.get(word) {
                if let Some(s) = w.get(&reversed) {
                    count = *f.min(&s);
                    ans += (count * 4);
                }
            }
            if count != 0 {
                w.entry(word.to_string()).and_modify(|c| *c -= count);
                w.entry(reversed.to_string()).and_modify(|c| *c -= count);
            }
        }

        ans
    }
}
