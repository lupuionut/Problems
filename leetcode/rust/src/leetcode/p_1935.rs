// 1935. Maximum Number of Words You Can Type
// ------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let words: Vec<&str> = text.split(' ').collect();
        let mut ans = 0;
        let mut banned = HashSet::new();
        broken_letters.as_bytes().iter().for_each(|b| {
            banned.insert(b);
        });

        words.iter().for_each(|w| {
            let mut possible = true;
            for b in w.as_bytes() {
                if banned.contains(b) {
                    possible = false;
                    break;
                }
            }
            if possible == true {
                ans += 1;
            }
        });

        ans
    }
}
