// 1420. Build Array Where You Can Find The Maximum Exactly K Comparisons
// ----------------------------------------------------------------------

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut cache = vec![vec![vec![-1; (k + 1) as usize]; (m + 1) as usize]; (n + 1) as usize];

        fn count(n: i32, mx: i32, m: i32, k: i32, cache: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if n == 0 {
                if k == 0 {
                    return 1;
                }
                return 0;
            }

            if cache[n as usize][mx as usize][k as usize] != -1 {
                return cache[n as usize][mx as usize][k as usize];
            }

            let mut ans = 0;

            for i in 1..=mx {
                ans += (count(n - 1, mx, m, k, cache) as i64 % 1_000_000_007);
            }

            if k > 0 {
                for i in (mx + 1)..=m {
                    ans += (count(n - 1, i, m, k - 1, cache) as i64 % 1_000_000_007);
                }
            }

            cache[n as usize][mx as usize][k as usize] = (ans % 1_000_000_007) as i32;
            cache[n as usize][mx as usize][k as usize]
        }

        count(n, 0, m, k, &mut cache)
    }
}
