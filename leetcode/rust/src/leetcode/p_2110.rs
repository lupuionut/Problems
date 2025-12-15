// 2110. Number of Smooth Descent Periods of a Stock
// -------------------------------------------------
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut streak = 0i64;
        let mut ans = 0;
        let mut curr = prices[0] + 1;
        for i in 0..prices.len() {
            if prices[i] + 1 == curr {
                streak += 1;
            } else {
                ans += (streak * (streak + 1)) / 2;
                streak = 1;
            }
            curr = prices[i];
        }
        ans += (streak * (streak + 1)) / 2;
        ans
    }
}
