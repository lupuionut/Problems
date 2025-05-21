// 73. Set Matrix Zeroes
// ---------------------

use std::collections::HashSet;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // contains zero (rows, cols)
        let mut zeros = (HashSet::new(), HashSet::new());
        let rows = matrix.len();
        let cols = matrix[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == 0 {
                    zeros.0.insert(i);
                    zeros.1.insert(j);
                }
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                if zeros.0.contains(&i) || zeros.1.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
