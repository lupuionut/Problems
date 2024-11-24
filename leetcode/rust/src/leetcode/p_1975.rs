// 1975. Maximum Matrix Sum
// ------------------------

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;
        let mut absmin = i32::MAX;
        let mut parity = 0;
        let n = matrix.len();
        for i in 0..n {
            for j in 0..n {
                ans += matrix[i][j].abs() as i64;
                if matrix[i][j] < 0 {
                    parity += 1;
                }
                absmin = absmin.min(matrix[i][j].abs());
            }
        }

        if parity % 2 == 0 {
            ans
        } else {
            ans - (2 * absmin as i64)
        }
    }
}
