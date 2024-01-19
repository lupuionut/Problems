// 931. Minimum Falling Path Sum
// -----------------------------

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        fn min_falling(
            row: usize,
            col: usize,
            matrix: &Vec<Vec<i32>>,
            cache: &mut Vec<Vec<Option<i32>>>,
        ) -> i32 {
            if cache[row][col] != None {
                return cache[row][col].unwrap();
            }

            let mut curr_min = matrix[row][col];
            if row < matrix.len() - 1 {
                let mut next = min_falling(row + 1, col, matrix, cache);
                if col >= 1 && col < matrix[0].len() {
                    next = next.min(min_falling(row + 1, col - 1, matrix, cache));
                }
                if col < matrix[0].len() - 1 {
                    next = next.min(min_falling(row + 1, col + 1, matrix, cache));
                }
                curr_min += next;
            }
            cache[row][col] = Some(curr_min);
            curr_min
        }

        let mut ans = i32::MAX;
        let mut cache = vec![vec![None; matrix[0].len()]; matrix.len()];
        for col in 0..matrix[0].len() {
            ans = ans.min(min_falling(0, col, &matrix, &mut cache));
        }
        ans
    }
}
