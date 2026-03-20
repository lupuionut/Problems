// 3567. Minimum Absolute Difference in Sliding Submatrix
// ------------------------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let k = k as usize;
        let mut ans = vec![vec![0; cols - k + 1]; rows - k + 1];

        for row in 0..rows - k + 1 {
            for col in 0..cols - k + 1 {
                let mut values = HashSet::new();
                for i in row..row + k {
                    for j in col..col + k {
                        values.insert(grid[i][j]);
                    }
                }
                let mut values = values.into_iter().collect::<Vec<i32>>();
                values.sort();
                let mut best = i32::MAX;
                if values.len() == 1 {
                    best = 0;
                } else {
                    for i in 1..values.len() {
                        best = best.min(values[i] - values[i - 1]);
                    }
                }

                ans[row][col] = best;
            }
        }

        ans
    }
}
