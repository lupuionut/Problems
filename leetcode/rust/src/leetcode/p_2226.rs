// 2226. Maximum Candies Allocated to K Children
// ---------------------------------------------

impl Solution {
    pub fn maximum_candies(mut candies: Vec<i32>, k: i64) -> i32 {
        let mut l = 1;
        candies.sort_by(|a, b| b.cmp(&a));
        let mut r = candies[0];

        fn is_good(mid: i32, candies: &Vec<i32>, mut k: i64) -> bool {
            if mid == 0 {
                return false;
            }
            for i in 0..candies.len() {
                if k <= 0 {
                    return true;
                }

                if candies[i] < mid {
                    return false;
                }

                k -= (candies[i] / mid) as i64;
            }
            if k > 0 {
                return false;
            }
            true
        }

        let mut best = 0;
        while l <= r {
            let mid = (l + r) / 2;
            if is_good(mid, &candies, k) {
                best = best.max(mid as i32);
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        best
    }
}
