// 1870. Minimum Speed to Arrive on Time
// -------------------------------------

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let min = dist.len() as f64;

        if hour <= min - 1.0 {
            return -1;
        }

        let mut left = 1;
        let mut right = std::i32::MAX;
        let mut ans = right;

        fn time_used(speed: i32, dist: &Vec<i32>) -> f64 {
            let mut ans = 0.0;
            let speed = speed as f64;
            let n = dist.len();
            for i in 0..n {
                let mut t = dist[i] as f64 / speed;
                if i != n - 1 {
                    t = t.ceil();
                }
                ans += t;
            }
            ans
        }

        while left <= right {
            let mid = left + (right - left) / 2;
            let t = time_used(mid, &dist);
            if t > hour {
                left = mid + 1;
            } else {
                ans = ans.min(mid);
                right = mid - 1;
            }
        }

        ans as i32
    }
}
