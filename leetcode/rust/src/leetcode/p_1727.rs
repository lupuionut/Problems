// 1727. Largest Submatrix With Rearrangements
// -------------------------------------------

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut col_max_ones = matrix.clone();

        for i in 1..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 1 {
                    col_max_ones[i][j] = col_max_ones[i - 1][j] + 1;
                }
            }
        }

        let mut ans = 0;
        for i in 0..col_max_ones.len() {
            let mut row = &mut col_max_ones[i];
            row.sort_by(|a, b| b.cmp(a));
            for (key, val) in row.iter().enumerate() {
                ans = ans.max((key as i32 + 1) * val);
            }
        }

        ans
    }
}
