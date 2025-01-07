// 1408. String Matching in an Array
// ---------------------------------

impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let mut ans = vec![];
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[j].contains(&words[i]) {
                    ans.push(words[i].clone());
                    break;
                }
            }
        }
        ans
    }
}
