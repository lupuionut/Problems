// 2706. Buy Two Chocolates
// ------------------------

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort();
        let rest = money - (prices[0] + prices[1]);
        if rest < 0 {
            return money;
        }
        rest
    }
}
