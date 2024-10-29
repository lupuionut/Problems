// 2684. Maximum Number of Moves in a Grid
// ---------------------------------------

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut cache = vec![vec![-1; grid[0].len()]; grid.len()];

        fn visit(
            previous: i32,
            row: usize,
            col: usize,
            grid: &Vec<Vec<i32>>,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if cache[row][col] != -1 {
                return cache[row][col];
            }

            if grid[row][col] <= previous {
                return 0;
            }

            let mut ans = 1;
            let mut best = 0;
            if row >= 1 && (col + 1) < grid[0].len() {
                best = best.max(visit(grid[row][col], row - 1, col + 1, grid, cache));
            }
            if ((col + 1) < grid[0].len()) {
                best = best.max(visit(grid[row][col], row, col + 1, grid, cache));
            }
            if (row + 1) < grid.len() && (col + 1) < grid[0].len() {
                best = best.max(visit(grid[row][col], row + 1, col + 1, grid, cache));
            }
            ans += best;
            cache[row][col] = ans;
            ans
        }

        let mut best = 1;
        for i in 0..grid.len() {
            if cache[i][0] == -1 {
                best = best.max(visit(-1, i, 0, &grid, &mut cache));
            }
        }

        best - 1
    }
}
