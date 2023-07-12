// 566. Reshape the Matrix
// -----------------------

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();

        if (r * c) != (m * n) as i32 {
            return mat;
        }

        let mut ans = vec![vec![0; c as usize]; r as usize];
        let mut row = 0;
        let mut col = 0;

        for i in 0..n {
            for j in 0..m {
                if col >= c as usize {
                    row += 1;
                    col = col % c as usize;
                }
                ans[row][col] = mat[i][j];
                col += 1;
            }
        }

        ans
    }
}
