// 1582. Special Positions in a Binary Matrix
// ------------------------------------------

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut total_row = vec![0; mat.len()];
        let mut total_col = vec![0; mat[0].len()];
        let mut ans = 0;

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                total_row[i] += mat[i][j];
                total_col[j] += mat[i][j];
            }
        }

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 && total_row[i] == 1 && total_col[j] == 1 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
