// 3789. Minimum Cost to Acquire Required Items
// --------------------------------------------

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let mut ans = 0;
        let mut total_need = need1 + need2;
        let mut min_both = need1.min(need2);
        let mut need1 = need1;
        let mut need2 = need2;
        if cost_both <= cost1 + cost2 {
            ans += (min_both as i64) * cost_both as i64;
            need1 -= min_both;
            need2 -= min_both;
        } 

        let mut cost1 = cost1;
        let mut cost2 = cost2;
        if cost_both < cost1 {
            cost1 = cost_both;
        } 
        if cost_both < cost2 {
            cost2 = cost_both;
        }
        ans += (need1 as i64) * (cost1 as i64);
        ans += (need2 as i64) * (cost2 as i64);

        ans
    }
}
