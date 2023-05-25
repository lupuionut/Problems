// 837. New 21 Game
// Based on https://www.youtube.com/watch?v=2wCShtvcP0Q

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let mut result = 0.0;
        let points = max_pts as f64;

        if k == 0 || n >= k + max_pts {
            return 1.0;
        }
        let mut dp: Vec<f64> = vec![0.0; (n + 1) as usize];
        dp[0] = 1.0;
        let mut running_sum = dp[0];

        for i in 1..n + 1 {
            dp[i as usize] = running_sum / points;

            if i < k {
                running_sum += dp[i as usize];
            } else {
                result += dp[i as usize];
            }

            if i - max_pts >= 0 {
                running_sum -= dp[(i - max_pts) as usize];
            }
        }

        result
    }
}
