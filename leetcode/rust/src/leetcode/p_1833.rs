// 1833. Maximum Ice Cream Bars
// ----------------------------
impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_by(|a,b| b.cmp(&a));
        let mut ans = 0;
        while let Some(c) = costs.pop() {
            if c > coins {
                break;
            }
            coins -= c;
            ans += 1;
        } 
        ans
    }
}
