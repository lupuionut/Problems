// 2328. Number of Increasing Paths in a Grid
// ------------------------------------------

impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut cache: Vec<Vec<i64>> = vec![vec![-1; n]; m];
        let mut counter = 0;

        fn count(i: usize, j: usize, grid: &Vec<Vec<i32>>, cache: &mut Vec<Vec<i64>>) -> i64 {
            if cache[i][j] != -1 {
                return cache[i][j];
            }

            let mut counter = 1;
            let directions: &[(i32, i32)] = &[(0, 1), (-1, 0), (0, -1), (1, 0)];
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;
            for (x, y) in directions {
                let ii = i as i32 + x;
                let jj = j as i32 + y;
                if ii < m
                    && ii >= 0
                    && jj < n
                    && jj >= 0
                    && grid[i as usize][j as usize] < grid[ii as usize][jj as usize]
                {
                    counter += count(ii as usize, jj as usize, grid, cache) % 1_000_000_007;
                }
            }
            cache[i][j] = counter % 1_000_000_007;
            cache[i][j]
        }

        for i in 0..m {
            for j in 0..n {
                counter += count(i, j, &grid, &mut cache) % 1_000_000_007;
            }
        }

        (counter % 1_000_000_007) as i32
    }
}
