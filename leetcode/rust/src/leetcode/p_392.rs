// 392. Is Subsequence
// -------------------

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;

        for j in 0..t.len() {
            if i == s.len() {
                return true;
            }
            if s[i] == t[j] {
                i += 1;
            }
        }

        if i == s.len() {
            return true;
        }
        false
    }
}
