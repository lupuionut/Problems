// 1380. Lucky Numbers in a Matrix
// -------------------------------

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rows_min = vec![i32::MAX; matrix.len()];
        let mut cols_max = vec![i32::MIN; matrix[0].len()];
        let mut ans = vec![];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                rows_min[i] = rows_min[i].min(matrix[i][j]);
                cols_max[j] = cols_max[j].max(matrix[i][j]);
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == rows_min[i] && matrix[i][j] == cols_max[j] {
                    ans.push(matrix[i][j]);
                }
            }
        }

        ans
    }
}
