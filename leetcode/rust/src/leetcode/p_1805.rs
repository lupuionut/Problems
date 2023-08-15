// 1805. Number of Different Integers in a String
// ----------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut stack = String::new();
        let mut unique = HashSet::new();

        fn strip_leading_zero(s: &String) -> String {
            let mut s = s.as_str();
            while s.starts_with('0') && s.len() > 1 {
                s = &s[1..];
            }
            s.to_string()
        }

        word.chars().for_each(|c| {
            if c.is_numeric() {
                stack.push(c);
            } else {
                if stack.len() > 0 {
                    unique.insert(strip_leading_zero(&stack));
                    stack = String::new();
                }
            }
        });

        if stack.len() > 0 {
            unique.insert(strip_leading_zero(&stack));
        }
        unique.len() as i32
    }
}
