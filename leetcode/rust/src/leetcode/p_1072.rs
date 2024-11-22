// 1072. Flip Columns For Maximum Number of Equal Rows
// ---------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut count = HashMap::new();
        let mut ans = 0;
        for i in 0..matrix.len() {
            let mut hash = "".to_string();
            if matrix[i][0] == 0 {
                hash = matrix[i]
                    .iter()
                    .map(|&n| if n == 0 { '1' } else { '0' })
                    .collect::<String>();
            } else {
                hash = matrix[i].iter().map(|n| n.to_string()).collect::<String>();
            }

            count.entry(hash).and_modify(|c| *c += 1).or_insert(1);
        }

        for (k, &v) in count.iter() {
            ans = ans.max(v);
        }

        ans
    }
}
