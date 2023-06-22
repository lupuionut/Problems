// 714. Best Time to Buy and Sell Stock with Transaction Fee
// ---------------------------------------------------------

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cache: Vec<Vec<i32>> = vec![vec![-1; 2]; prices.len()];

        fn dp(
            i: usize,
            state: usize,
            prices: &Vec<i32>,
            fee: i32,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i == prices.len() {
                return 0;
            }
            if cache[i][state] != -1 {
                return cache[i][state];
            }

            if state == 0 {
                let buy = -prices[i] + dp(i + 1, 1, prices, fee, cache);
                let skip = dp(i + 1, 0, prices, fee, cache);
                cache[i][state] = buy.max(skip);
            } else {
                let sell = prices[i] - fee + dp(i + 1, 0, prices, fee, cache);
                let skip = dp(i + 1, 1, prices, fee, cache);
                cache[i][state] = sell.max(skip);
            }

            cache[i][state]
        }

        dp(0, 0, &prices, fee, &mut cache)
    }
}

/*

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut no_stock = 0;
        let mut stock = -prices[0];

        for i in 0..prices.len() {
            no_stock = no_stock.max(stock + prices[i] - fee);
            stock = stock.max(no_stock - prices[i]);
        }

        no_stock
    }
}

*/
