// 2577. Minimum Time to Visit a Cell In a Grid
// --------------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut heap = BinaryHeap::new();
        let mut distances = vec![vec![i32::MAX; m]; n];
        heap.push((0, (0, 0)));
        distances[0][0] = 0;

        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        while heap.len() > 0 {
            if let Some((d, (x, y))) = heap.pop() {
                if -d > distances[x as usize][y as usize] {
                    continue;
                }
                for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                        continue;
                    }
                    let mut cost = if -d >= grid[nx as usize][ny as usize] {
                        0
                    } else {
                        grid[nx as usize][ny as usize] + d
                    };
                    if cost % 2 == 0 {
                        cost += 1;
                    }
                    let nd = d - cost;
                    if -nd < distances[nx as usize][ny as usize] {
                        distances[nx as usize][ny as usize] = -nd;
                        heap.push((nd, (nx, ny)));
                    }
                }
            }
        }
        if distances[n - 1][m - 1] == i32::MAX {
            return -1;
        }
        distances[n - 1][m - 1]
    }
}
