// 1475. Final Prices With a Special Discount in a Shop
// ----------------------------------------------------

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut ans = prices.clone();
        for i in 0..ans.len() {
            for j in i + 1..ans.len() {
                if ans[j] <= ans[i] {
                    ans[i] -= ans[j];
                    break;
                }
            }
        }
        ans
    }
}
