// 1957. Delete Characters to Make Fancy String
// --------------------------------------------

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = String::new();
        let s = s.into_bytes();
        for i in 0..s.len() {
            if i == 0 || i == s.len() - 1 || s[i - 1] ^ s[i] != 0 || s[i + 1] ^ s[i] != 0 {
                ans.push(s[i] as char);
            }
        }
        ans
    }
}
