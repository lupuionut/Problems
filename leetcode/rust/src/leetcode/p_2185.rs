// 2185. Counting Words With a Given Prefix
// ----------------------------------------

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut ans = 0;
        words.iter().for_each(|w| {
            if w.starts_with(&pref) {
                ans += 1;
            }
        });
        ans
    }
}
