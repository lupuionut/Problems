// 3147. Taking Maximum Energy From the Mystic Dungeon
// ---------------------------------------------------
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut dp = energy.clone();
        for i in 0..energy.len() {
            if i as i32 - k >= 0 {
                dp[i] = dp[i].max(dp[i] + dp[i - k as usize]);
            }
        }
        let mut ans = -1000;
        for i in (dp.len() - k as usize..dp.len()).rev() {
            ans = ans.max(dp[i])
        }
        ans
    }
}
