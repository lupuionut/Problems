// 63. Unique Paths II
// -------------------

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut cache = vec![vec![-1; obstacle_grid[0].len()]; obstacle_grid.len()];

        fn dfs(i: usize, j: usize, grid: &Vec<Vec<i32>>, cache: &mut Vec<Vec<i32>>) -> i32 {
            if i == grid.len() - 1 && j == grid[0].len() - 1 {
                if grid[i][j] == 1 {
                    return 0;
                }
                return 1;
            }
            if i == grid.len() || j == grid[0].len() || grid[i][j] == 1 {
                return 0;
            }
            if cache[i][j] != -1 {
                return cache[i][j];
            }
            cache[i][j] = dfs(i + 1, j, grid, cache) + dfs(i, j + 1, grid, cache);
            cache[i][j]
        }
        dfs(0, 0, &obstacle_grid, &mut cache)
    }
}
