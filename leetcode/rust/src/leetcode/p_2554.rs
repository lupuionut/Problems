// 2554. Maximum Number of Integers to Choose From a Range I
// ---------------------------------------------------------

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut possible = vec![true; (n + 1) as usize];
        let mut curr_sum = 0;
        let mut ans = 0;
        for i in 0..banned.len() {
            if banned[i] <= n {
                possible[banned[i] as usize] = false;
            }
        }

        for i in 1..possible.len() {
            if (curr_sum + i as i64) > max_sum as i64 {
                break;
            }
            if possible[i] == true && (curr_sum + i as i64) <= max_sum as i64 {
                curr_sum += i as i64;
                ans += 1;
            }
        }

        ans
    }
}
