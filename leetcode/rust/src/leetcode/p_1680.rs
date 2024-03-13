// 1680. Concatenation of Consecutive Binary Numbers
// -------------------------------------------------

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut ans: i64 = 0;
        for i in 1..=n {
            let len = format!("{i:b}").len();
            ans = ans << len;
            ans |= i as i64;
            ans %= 1_000_000_007;
        }
        ans as i32
    }
}
