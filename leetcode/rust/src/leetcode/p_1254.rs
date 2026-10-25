// 1254. Number of Closed Islands
// ------------------------------
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let c = grid[0].len();
        let r = grid.len();
        let mut visited = vec![vec![false; c]; r];
        let mut ans = 0;

        fn visit(grid: &Vec<Vec<i32>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
            let mut q = vec![];
            q.push((i, j));
            let mut island = true;
            
            while let Some((i, j)) = q.pop() {
                visited[i][j] = true;
                if i == 0 || j == 0 {
                    island = false;
                }
                if i == grid.len() - 1 || j == grid[0].len() - 1 {
                    island = false;
                } 

                let mut nd = vec![];
                if j > 0 {
                    nd.push((i, j - 1));
                }
                if i > 0 {
                    nd.push((i - 1, j));
                }
                if j < grid[0].len() - 1 {
                    nd.push((i, j + 1));
                }
                if i < grid.len() - 1 {
                    nd.push((i + 1, j));
                }
                for &(ni, nj) in &nd {
                    if visited[ni][nj] == false && grid[ni][nj] == 0 {
                        q.push((ni, nj));
                    }
                }                
            }

            if island {
                return 1;
            }
            0
        }

        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 0 && visited[i][j] == false {
                    ans += visit(&grid, i, j, &mut visited);
                }
            }
        }

        ans
    }
}
