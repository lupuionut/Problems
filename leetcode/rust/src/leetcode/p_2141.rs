// 2141. Maximum Running Time of N Computers
// -----------------------------------------

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut batteries = batteries;
        let n = n as i64;
        batteries.sort_by(|a, b| b.cmp(&a));

        let mut reserve: i64 = 0;
        batteries[n as usize..].iter().for_each(|x| {
            reserve += *x as i64;
        });
        let mut left = 1;
        let mut right: i64 = 0;
        batteries.iter().for_each(|x| {
            right += *x as i64;
        });
        right = right / n;
        let mut ans = 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            let mut total_power = reserve;
            batteries[..n as usize].iter().for_each(|b| {
                let b = *b as i64;
                total_power += b.min(mid);
            });
            if total_power / n >= mid {
                ans = ans.max(mid);
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}
