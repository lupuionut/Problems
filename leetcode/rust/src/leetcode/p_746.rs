// 746. Min Cost Climbing Stairs
// -----------------------------

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; 2];

        for i in 0..n {
            if i >= 2 {
                dp[i % 2] = cost[i] + dp[(i - 1) % 2].min(dp[(i - 2) % 2]);
            } else {
                dp[i] = cost[i];
            }
        }

        dp[0].min(dp[1])
    }
}
