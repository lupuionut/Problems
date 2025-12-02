// 3623. Count Number of Trapezoids I
// ----------------------------------
use std::collections::HashMap;
impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut ys: HashMap<i32, i64> = HashMap::new();
        let mut ans = 0i64;
        for i in 0..n {
            ys.entry(points[i][1]).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut total_pairs = 0;
        for (&k, &v) in ys.iter() {
            total_pairs += (v * (v - 1)) / 2;
        }

        for (&k, &v) in ys.iter() {
            let curr_pairs = (v * (v - 1)) / 2;
            ans += curr_pairs * (total_pairs - curr_pairs);
            total_pairs -= curr_pairs;
            ans %= 1_000_000_007;
        }

        ans as i32
    }
}
