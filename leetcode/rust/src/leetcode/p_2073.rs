// 2073. Time Needed to Buy Tickets
// --------------------------------

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let val = tickets[k as usize];
        let mut ans = 0;
        for i in 0..tickets.len() {
            if i < k as usize {
                ans += tickets[i].min(val);
            } else if i == k as usize {
                ans += val;
            } else {
                ans += tickets[i].min(val - 1);
            }
        }
        ans
    }
}
