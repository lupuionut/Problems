// 2938. Separate Black and White Balls
// ------------------------------------

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut ans = 0;
        let mut ones = 0i64;
        s.chars().for_each(|c| {
            if c == '1' {
                ones += 1;
            } else {
                ans += ones;
            }
        });
        ans
    }
}
