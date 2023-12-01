// 1662. Check If Two String Arrays are Equivalent
// -----------------------------------------------

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let s1 = word1.iter().fold("".to_string(), |mut acc, w| {
            acc.push_str(w);
            acc
        });
        let s2 = word2.iter().fold("".to_string(), |mut acc, w| {
            acc.push_str(w);
            acc
        });

        s1 == s2
    }
}
