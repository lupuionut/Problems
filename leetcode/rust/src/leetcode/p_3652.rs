// 3652. Best Time to Buy and Sell Stock using Strategy
// ----------------------------------------------------
impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let mut ps_sell: Vec<i64> = prices.clone().iter().map(|&n| n as i64).collect();
        let mut ps: Vec<i64> = prices.clone().iter().map(|&n| n as i64).collect();
        ps[0] = ps[0] * strategy[0] as i64;

        for i in 1..prices.len() {
            ps_sell[i] = ps_sell[i - 1] + ps_sell[i];
            ps[i] = ps[i - 1] + (ps[i] * strategy[i] as i64);
        }

        let k = k as usize;
        let n = prices.len();
        let mut best = ps[n - 1];
        for i in k - 1..n {
            let mut t = 0;
            let first = i - (k - 1);
            if first > 0 {
                t += ps[first - 1];
            }
            t += ps[n - 1] - ps[i];
            t += ps_sell[i] - ps_sell[i - k / 2];
            best = best.max(t);
        }

        best
    }
}
