// 2942. Find Words Containing Character
// -------------------------------------

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ans = vec![];
        words.iter().enumerate().for_each(|(k, w)| {
            if w.contains(x) {
                ans.push(k as i32);
            }
        });
        ans
    }
}
