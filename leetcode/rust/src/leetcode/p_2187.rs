// 2187. Minimum Time to Complete Trips
// ------------------------------------

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut l: i64 = *time.iter().min().unwrap() as i64;
        let mut r: i64 = *time.iter().max().unwrap() as i64 * total_trips as i64;
        let mut ans = r;

        while l <= r {
            let middle = (l + r) / 2;
            let total: i64 = time.iter().fold(0, |acc, v| acc + (middle / (*v as i64)));
            if total < total_trips.into() {
                l = middle + 1;
            } else {
                r = middle - 1;
                ans = ans.min(middle);
            }
        }

        ans
    }
}
