// 576. Out of Boundary Paths
// --------------------------

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        fn moves(
            row: i32,
            col: i32,
            m: i32,
            n: i32,
            remain: i32,
            cache: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if row < 0 || row >= m {
                return 1;
            }
            if col < 0 || col >= n {
                return 1;
            }
            if remain == 0 {
                return 0;
            }
            if cache[row as usize][col as usize][remain as usize] != -1 {
                return cache[row as usize][col as usize][remain as usize];
            }
            let mut total = 0;
            if (row >= 0 && row < m) {
                total += moves(row - 1, col, m, n, remain - 1, cache) % 1_000_000_007;
                total %= 1_000_000_007;
                total += moves(row + 1, col, m, n, remain - 1, cache) % 1_000_000_007;
                total %= 1_000_000_007;
            }
            if (col >= 0 && col < n) {
                total += moves(row, col - 1, m, n, remain - 1, cache) % 1_000_000_007;
                total %= 1_000_000_007;
                total += moves(row, col + 1, m, n, remain - 1, cache) % 1_000_000_007;
                total %= 1_000_000_007;
            }
            cache[row as usize][col as usize][remain as usize] = total % 1_000_000_007;
            cache[row as usize][col as usize][remain as usize]
        }

        let mut cache = vec![vec![vec![-1; (max_move + 1) as usize]; n as usize]; m as usize];
        moves(start_row, start_column, m, n, max_move, &mut cache)
    }
}
