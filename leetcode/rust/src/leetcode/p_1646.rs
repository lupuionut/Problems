// 1646. Get Maximum in Generated Array
// ------------------------------------

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 0;
        dp[1] = 1;
        let mut ans = 1;
        let mut i = 1;

        loop {
            let res = dp[i as usize];
            dp[(2 * i) as usize] = res;
            ans = ans.max(res);
            if (i * 2) == n {
                break;
            }

            let res = dp[i as usize] + dp[(i + 1) as usize];
            dp[(i * 2 + 1) as usize] = res;
            ans = ans.max(res);
            if (i * 2 + 1) == n {
                break;
            }

            i += 1;
        }
        ans
    }
}
