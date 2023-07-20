// 1550. Three Consecutive Odds
// ----------------------------

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut n = 0;

        if arr.len() < 3 {
            return false;
        }

        n = ((arr[0] % 2) << 2) | n;
        n = ((arr[1] % 2) << 1) | n;

        for i in 2..arr.len() {
            if i >= 3 {
                n = (n & 3) << 1;
            }
            n = ((arr[i] % 2) << 0) | n;
            if n == 7 {
                return true;
            }
        }

        false
    }
}
