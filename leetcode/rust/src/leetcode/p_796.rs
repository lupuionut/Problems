// 796. Rotate String
// ------------------

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s == goal {
            return true;
        }

        for i in 1..s.len() {
            let mut ss = String::new();
            ss.push_str(&s[i..]);
            ss.push_str(&s[0..i]);
            if ss == goal {
                return true;
            }
        }
        false
    }
}
