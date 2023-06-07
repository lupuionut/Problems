// 200. Number of Islands
// ----------------------

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut counter = 0;
        let m = grid.len();
        let n = grid[0].len();
        let mut visited: Vec<Vec<i32>> = vec![vec![0; n]; m];

        fn dfs(i: usize, j: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>) {
            if i < 0 || i >= grid.len() {
                return;
            }
            if j < 0 || j >= grid[0].len() {
                return;
            }
            if visited[i][j] == 1 || grid[i][j] == '0' {
                return;
            }
            visited[i][j] = 1;
            dfs(i - 1, j, grid, visited);
            dfs(i + 1, j, grid, visited);
            dfs(i, j - 1, grid, visited);
            dfs(i, j + 1, grid, visited);
        }

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' && visited[i][j] == 0 {
                    counter += 1;
                    dfs(i, j, &grid, &mut visited);
                }
            }
        }

        counter
    }
}
