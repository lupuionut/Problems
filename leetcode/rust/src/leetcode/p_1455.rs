// 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence
// ------------------------------------------------------------------

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words = sentence.split(' ').collect::<Vec<_>>();
        for i in 0..words.len() {
            if words[i].starts_with(search_word.as_str()) {
                return (i + 1) as i32;
            }
        }
        -1
    }
}
