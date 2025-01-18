// 1368. Minimum Cost to Make at Least One Valid Path in a Grid
// ------------------------------------------------------------

use std::collections::VecDeque;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        let r = grid.len();
        let c = grid[0].len();
        let directions = &[(1, 0, 1), (2, 0, -1), (3, 1, 0), (4, -1, 0)];
        let mut distances = vec![vec![i32::MAX; c]; r];

        q.push_back((0, 0));
        distances[0][0] = 0;

        while q.len() > 0 {
            if let Some((x, y)) = q.pop_front() {
                for &(dd, dx, dy) in directions {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= r as i32 || ny < 0 || ny >= c as i32 {
                        continue;
                    }
                    let nc = if dd == grid[x as usize][y as usize] {
                        0
                    } else {
                        1
                    };
                    if distances[nx as usize][ny as usize] - nc > distances[x as usize][y as usize]
                    {
                        distances[nx as usize][ny as usize] =
                            distances[x as usize][y as usize] + nc;
                        if nc == 0 {
                            q.push_front((nx, ny));
                        } else {
                            q.push_back((nx, ny));
                        }
                    }
                }
            } else {
                break;
            }
        }

        distances[r - 1][c - 1]
    }
}
