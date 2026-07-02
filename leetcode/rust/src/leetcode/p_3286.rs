// 3286. Find a Safe Walk Through a Grid
// -------------------------------------
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let mut h = BinaryHeap::new();
        h.push((health - grid[0][0], 0, 0));
        let m = grid[0].len();
        let n = grid.len();
        let d = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut visited = vec![vec![false; m]; n];
        visited[0][0] = true;

        while let Some((hth, i, j)) = h.pop() {
            if i == n - 1 && j == m - 1 {
                return true;
            }
            for &(di, dj) in &d {
                let ni = di + i as i32;
                let nj = dj + j as i32;
                if ni < 0 || nj < 0 || ni >= n as i32 || nj >= m as i32 {
                    continue;
                }
                if visited[ni as usize][nj as usize] == true {
                    continue;
                }
                if hth - grid[ni as usize][nj as usize] <= 0 {
                    continue;
                }
                visited[ni as usize][nj as usize] = true;
                h.push((hth - grid[ni as usize][nj as usize], ni as usize, nj as usize));
            }
        }
        false
    }
}
