// 3228. Maximum Number of Operations to Move Ones to the End
// ----------------------------------------------------------
impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut ans = 0;
        let mut chars: Vec<char> = s.chars().collect();
        chars.push('1');
        let n = chars.len();
        let mut ones = 0;

        for i in 0..n {
            if chars[i] == '1' {
                if i > 0 && chars[i - 1] == '0' {
                    ans += ones;
                }
                ones += 1;
            }
        }
        ans
    }
}
