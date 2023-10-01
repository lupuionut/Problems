// 557. Reverse Words in a String III
// ----------------------------------

impl Solution {
    pub fn reverse_words(s: String) -> String {
        fn reverse(s: &str) -> String {
            s.chars().rev().collect::<String>()
        }

        s.split(' ')
            .map(|w| reverse(w))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
