// 1550. Three Consecutive Odds
// ----------------------------

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for i in 1..arr.len() - 1 {
            if (arr[i - 1] & arr[i] & arr[i + 1]) & 1 == 1 {
                return true;
            }
        }
        false
    }
}
