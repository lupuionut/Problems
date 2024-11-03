// 796. Rotate String
// ------------------

use std::collections::VecDeque;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let mut crs = VecDeque::new();
        s.chars().for_each(|c| {
            crs.push_back(c);
        });
        let mut n = crs.len();

        while n > 0 {
            let first = crs.pop_front().unwrap();
            crs.push_back(first);
            let s = crs.iter().collect::<String>();
            if s == goal {
                return true;
            }
            n -= 1;
        }

        false
    }
}
