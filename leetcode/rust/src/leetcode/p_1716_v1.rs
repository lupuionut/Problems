// 1716. Calculate Money in Leetcode Bank
// --------------------------------------

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut ans = 0;
        let weeks = n / 7;
        let mut days = n % 7;
        // 28 + (7 * 0)  + 28 + (7 * 1) + ... 28 + (7 * weeks)
        // weeks * 28 + 7 * (0 + 1 + .. + weeks)
        ans += weeks * 28 + 7 * ((weeks - 1) * weeks / 2);
        for i in 1..=days {
            ans += (i + weeks);
        }
        ans
    }
}
