// 2483. Minimum Penalty for a Shop
// --------------------------------

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let n = customers.len();
        let mut ps_y = vec![0; n + 1];
        let mut ps_n = vec![0; n + 1];
        let mut bytes = customers.as_bytes();
        // ans.0 = idx
        // ans.1 = val
        let mut ans = (n + 1, n + 1);
        let mut ns = 0;

        for i in 1..=n {
            if bytes[i - 1] == b'Y' {
                ps_y[i] = ps_y[i - 1] + 1;
                ps_n[i] = ps_n[i - 1];
            } else {
                ps_n[i] = ps_n[i - 1] + 1;
                ps_y[i] = ps_y[i - 1];
            }
        }

        for i in 0..=n {
            let score = ps_n[i] + (ps_y[n] - ps_y[i]);
            if score < ans.1 {
                ans = (i, score);
            }
        }

        ans.0 as i32
    }
}
