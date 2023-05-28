// 1625. Lexicographically Smallest String After Applying Operations
// -----------------------------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut stack: Vec<String> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        let mut smallest = s;
        stack.push(smallest.clone());

        while stack.len() > 0 {
            let current = stack.pop().unwrap();
            visited.insert(current.to_string());

            if (Solution::is_smaller(&current, smallest.as_str())) {
                smallest = current.clone();
            }

            // increase
            let news = Solution::increase(&current, a);
            if !visited.contains(&news) {
                stack.push(news);
            }

            // shift
            let shifted = Solution::shift(&current, b);
            if !visited.contains(&shifted) {
                stack.push(shifted);
            }
        }

        return smallest.to_string();
    }

    pub fn is_smaller(s1: &str, s2: &str) -> bool {
        if s1.len() == 0 {
            return false;
        }
        let head1 = s1.chars().next().unwrap();
        let head2 = s2.chars().next().unwrap();

        if head1 < head2 {
            return true;
        }
        if head1 > head2 {
            return false;
        }
        return Solution::is_smaller(&s1[1..], &s2[1..]);
    }

    pub fn increase(s: &str, n: i32) -> String {
        let mut res = "".to_string();
        let mut chars = s.chars();
        let mut i = 0;
        while let Some(c) = chars.next() {
            if i % 2 == 1 {
                let d = (c.to_digit(10).unwrap() as i32 + n) % 10;
                res.push_str(&d.to_string());
            } else {
                res.push(c);
            }
            i += 1;
        }
        res
    }

    pub fn shift(s: &str, n: i32) -> String {
        let mut res = "".to_string();
        let start = s.len() - n as usize;
        res.push_str(&s[start..s.len()]);
        res.push_str(&s[0..start]);
        res
    }
}
