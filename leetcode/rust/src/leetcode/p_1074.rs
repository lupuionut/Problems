// 1074. Number of Submatrices That Sum to Target
// ----------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut prefix_sum = vec![vec![0; cols]; rows];

        for row in 0..rows {
            for col in 0..cols {
                prefix_sum[row][col] = matrix[row][col];
                if row > 0 {
                    prefix_sum[row][col] += prefix_sum[row - 1][col];
                }
                if col > 0 {
                    prefix_sum[row][col] += prefix_sum[row][col - 1];
                }
                if row > 0 && col > 0 {
                    prefix_sum[row][col] -= prefix_sum[row - 1][col - 1];
                }
            }
        }

        let mut ans = 0;
        for top in 0..rows {
            for bottom in top..rows {
                let mut counts: HashMap<i32, i32> = HashMap::new();
                counts.insert(0, 1);
                for col in 0..cols {
                    let mut val = prefix_sum[bottom][col];
                    if top > 0 {
                        val -= prefix_sum[top - 1][col];
                    }
                    let target = val - target;
                    if let Some(&val) = counts.get(&target) {
                        ans += val;
                    }
                    counts.entry(val).and_modify(|c| *c += 1).or_insert(1);
                }
            }
        }

        ans
    }
}
