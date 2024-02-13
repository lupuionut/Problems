// 2108. Find First Palindromic String in the Array
// ------------------------------------------------

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let possible = words
            .into_iter()
            .filter(|word| {
                let reversed = word.chars().rev().collect::<String>();
                reversed == *word
            })
            .collect::<Vec<String>>();

        if possible.len() > 0 {
            return possible[0].clone();
        }
        "".to_string()
    }
}
