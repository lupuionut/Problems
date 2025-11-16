// 1513. Number of Substrings With Only 1s
// ---------------------------------------

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut streak = 0;
        let mut groups = vec![];
        let mut ans = 0i64;
        s.chars().for_each(|c| {
            if c == '0' {
                if streak > 0 {
                    groups.push(streak);
                }
                streak = 0;
            } else {
                streak += 1;
            }
        });
        if streak > 0 {
            groups.push(streak);
        }
        for i in 0..groups.len() {
            ans += ((groups[i] as i64 * (groups[i] + 1) as i64) / 2) % 1_000_000_007;
        }
        (ans % 1_000_000_007) as i32
    }
}
