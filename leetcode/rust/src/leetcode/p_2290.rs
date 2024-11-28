// 2290. Minimum Obstacle Removal to Reach Corner
// ----------------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        queue.push_back((grid[0][0], 0, 0));
        visited[0][0] = true;

        while queue.len() > 0 {
            if let Some((d, x, y)) = queue.pop_front() {
                if x == rows - 1 && y == cols - 1 {
                    return d;
                }
                for (xx, yy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + xx;
                    let ny = y + yy;
                    if nx < 0 || nx >= rows || ny < 0 || ny >= cols {
                        continue;
                    }
                    if visited[nx as usize][ny as usize] == true {
                        continue;
                    }
                    visited[nx as usize][ny as usize] = true;
                    if grid[nx as usize][ny as usize] == 1 {
                        queue.push_back((d + 1, nx, ny));
                    } else {
                        queue.push_front((d, nx, ny));
                    }
                }
            }
        }

        -1
    }
}
