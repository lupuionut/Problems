// 1415. The k-th Lexicographical String of All Happy Strings of Length n
// ----------------------------------------------------------------------
use std::collections::VecDeque;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut ans = String::new();
        let mut q = VecDeque::new();
        q.push_back(("".to_string(), 'd'));
        let mut idx = 0;
        while q.len() > 0 {
            if let Some((curr, last)) = q.pop_front() {
                if curr.len() > n as usize {
                    break;
                }
                if curr.len() == n as usize {
                    idx += 1;
                    if idx == k {
                        return curr;
                        break;
                    }
                }
                for &c in &['a', 'b', 'c'] {
                    if c != last {
                        let mut new = curr.clone();
                        new.push(c);
                        q.push_back((new, c));
                    }
                }
            }
        }
        ans
    }
}
