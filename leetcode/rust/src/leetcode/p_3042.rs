// 3042. Count Prefix and Suffix Pairs I
// -------------------------------------

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut ans = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
