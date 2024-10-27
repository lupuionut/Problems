// 1277. Count Square Submatrices with All Ones
// --------------------------------------------

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0; cols]; rows];
        let mut ans = 0;

        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == 1 {
                    dp[i][j] = 1;
                    if i >= 1 && j >= 1 {
                        dp[i][j] = dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1])) + 1;
                    }
                    ans += dp[i][j];
                }
            }
        }

        ans
    }
}
