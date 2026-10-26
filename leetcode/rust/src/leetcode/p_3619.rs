// 3619. Count Islands With Total Value Divisible by K
// ---------------------------------------------------

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut ans = 0;

        fn visit(i: usize, j: usize, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i64 {
            let mut q = vec![];
            q.push((i, j));
            let mut t = 0;

            while let Some((i, j)) = q.pop() {
                t += grid[i][j] as i64;
                let mut d = vec![];
                if i > 0 {
                    d.push((i - 1, j));
                }
                if j > 0 {
                    d.push((i, j - 1));
                }
                if j < grid[0].len() - 1 {
                    d.push((i, j+1));
                }
                if i < grid.len() - 1 {
                    d.push((i+1, j));
                }

                for &(ni, nj) in &d {
                    if grid[ni][nj] > 0 && visited[ni][nj] == false {
                        q.push((ni, nj));
                        visited[ni][nj] = true;
                    }
                }
            }

            t 
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if visited[i][j] == false && grid[i][j] > 0 {
                    visited[i][j] = true;
                    let r = visit(i, j, &grid, &mut visited);
                    if r % k as i64 == 0 {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}
