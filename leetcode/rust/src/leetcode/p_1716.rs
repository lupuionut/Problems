// 1716. Calculate Money in Leetcode Bank
// --------------------------------------

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut idx = 0;
        let mut ans = 0;
        let mut i = 1;

        while i <= n {
            if i % 7 == 1 {
                idx = (i / 7) + 1;
            }
            ans += idx;
            i += 1;
            idx += 1;
        }

        ans
    }
}
