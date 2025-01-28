// 2658. Maximum Number of Fish in a Grid
// --------------------------------------

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid[0].len();
        let mut dp = vec![vec![0; c]; r];

        fn dfs(i: i32, j: i32, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
            if grid[i as usize][j as usize] == 0 {
                return 0;
            }
            visited[i as usize][j as usize] = true;
            let mut ans = grid[i as usize][j as usize];
            let mut t = ans;
            let directions = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
            for &(dx, dy) in directions {
                let nx = i + dx;
                let ny = j + dy;
                if nx < 0
                    || nx >= grid.len() as i32
                    || ny < 0
                    || ny >= grid[0].len() as i32
                    || visited[nx as usize][ny as usize] == true
                {
                    continue;
                }
                t = t.max(t + dfs(nx, ny, grid, visited));
            }

            ans.max(t)
        }

        let mut best = 0;
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] > 0 {
                    let mut visited = vec![vec![false; c]; r];
                    best = best.max(dfs(i as i32, j as i32, &grid, &mut visited));
                }
            }
        }

        best
    }
}
