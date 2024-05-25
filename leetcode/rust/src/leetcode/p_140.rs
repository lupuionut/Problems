// 140. Word Break II
// ------------------

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn compose(
            i: usize,
            s: &Vec<char>,
            curr: &mut Vec<String>,
            ans: &mut Vec<String>,
            words: &HashSet<String>,
        ) {
            if i == s.len() {
                let s = curr.join(" ").to_string();
                ans.push(s);
                return;
            }
            for j in i..=s.len() {
                let word = &s[i..j].into_iter().collect::<String>();
                if words.contains(word) {
                    curr.push(word.to_string());
                    compose(j, s, curr, ans, words);
                    curr.pop();
                }
            }
        }

        let mut words = HashSet::new();
        let s = s.chars().collect::<Vec<char>>();
        word_dict.into_iter().for_each(|w| {
            words.insert(w);
        });
        let mut ans = vec![];
        let mut curr = vec![];
        compose(0, &s, &mut curr, &mut ans, &words);
        ans
    }
}
