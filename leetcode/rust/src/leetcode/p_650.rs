// 650. 2 Keys Keyboard
// --------------------

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp = vec![0; (n + 1) as usize];
        dp[1] = 0;

        for i in 2..=n {
            dp[i as usize] = i;
            for j in 1..i {
                if i % j == 0 {
                    dp[i as usize] = dp[i as usize].min(dp[j as usize] + i / j);
                }
            }
        }

        dp[n as usize]
    }
}
