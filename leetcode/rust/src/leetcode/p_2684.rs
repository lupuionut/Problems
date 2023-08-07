// 2684. Maximum Number of Moves in a Grid
// ---------------------------------------

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut cache = vec![vec![-1; grid[0].len()]; grid.len()];

        fn dp(
            row: i32,
            col: i32,
            acc: i32,
            grid: &Vec<Vec<i32>>,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            let val = grid[row as usize][col as usize];
            let mut ans = acc;

            if cache[row as usize][col as usize] != -1 {
                return cache[row as usize][col as usize];
            }

            if row - 1 >= 0 && ((col + 1) as usize) < grid[0].len() {
                if grid[(row - 1) as usize][(col + 1) as usize] > val {
                    ans = ans.max(dp(row - 1, col + 1, acc + 1, grid, cache));
                }
            }

            if ((col + 1) as usize) < grid[0].len() {
                if grid[row as usize][(col + 1) as usize] > val {
                    ans = ans.max(dp(row, col + 1, acc + 1, grid, cache));
                }
            }

            if ((row + 1) as usize) < grid.len() && ((col + 1) as usize) < grid[0].len() {
                if grid[(row + 1) as usize][(col + 1) as usize] > val {
                    ans = ans.max(dp(row + 1, col + 1, acc + 1, grid, cache));
                }
            }

            cache[row as usize][col as usize] = ans;
            cache[row as usize][col as usize]
        }

        for i in 0..grid.len() {
            ans = ans.max(dp(i as i32, 0, 0, &grid, &mut cache));
        }

        ans
    }
}
