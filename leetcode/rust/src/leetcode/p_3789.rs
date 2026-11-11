// 3789. Minimum Cost to Acquire Required Items
// --------------------------------------------

impl Solution {
    pub fn minimum_cost(mut cost1: i32, mut cost2: i32, cost_both: i32, mut need1: i32, mut need2: i32) -> i64 {
        let mut ans = 0;
        let mut min_both = need1.min(need2);
        if cost_both <= cost1 + cost2 {
            ans += (min_both as i64) * cost_both as i64;
            need1 -= min_both;
            need2 -= min_both;
        } 
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
