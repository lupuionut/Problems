// 1805. Number of Different Integers in a String
// ----------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut stack = String::new();
        let mut unique = HashSet::new();

        word.chars().for_each(|c| {
            if c.is_numeric() {
                stack.push(c);
            } else {
                if stack.len() > 0 {
                    let mut s = stack.as_str();
                    while s.starts_with('0') && s.len() > 1 {
                        s = &s[1..];
                    }
                    unique.insert(s.to_string());
                    stack = String::new();
                }
            }
        });

        if stack.len() > 0 {
            let mut s = stack.as_str();
            while s.starts_with('0') && s.len() > 1 {
                s = &s[1..];
            }
            unique.insert(s.to_string());
        }
        unique.len() as i32
    }
}
