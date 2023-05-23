// 2485. Find the Pivot Integer
// ----------------------------

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut prefix_sum = vec![0; (n + 1) as usize];
        let mut ans = -1;

        for i in 1..n + 1 {
            prefix_sum[i as usize] = prefix_sum[(i - 1) as usize] + i;
        }

        for i in 1..n + 1 {
            let left = prefix_sum[i as usize];
            let right = prefix_sum[n as usize] - prefix_sum[(i - 1) as usize];
            if left == right {
                ans = i;
                break;
            }
        }

        ans
    }
}
