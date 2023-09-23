// 1048. Longest String Chain
// --------------------------

use std::collections::HashMap;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut ans = 1;
        let mut words = words;
        let mut word_chain: HashMap<String, i32> = HashMap::new();
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        words.iter().for_each(|word| {
            let mut longest = 1;
            for i in 0..word.len() {
                let mut subword = String::new();
                subword.push_str(&word[..i]);
                subword.push_str(&word[i + 1..]);
                if let Some(l) = word_chain.get(&subword) {
                    longest = longest.max(l + 1);
                }
            }
            word_chain.insert(word.to_string(), longest);
            ans = ans.max(longest);
        });

        ans
    }
}
