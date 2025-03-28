// 2503. Maximum Number of Points From Grid Queries
// ------------------------------------------------

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; queries.len()];
        let mut visited = HashSet::new();
        let mut qs = queries
            .iter()
            .enumerate()
            .map(|(k, &v)| (v, k))
            .collect::<Vec<_>>();
        qs.sort();
        let mut q = BinaryHeap::new();
        q.push((Reverse(grid[0][0]), 0i32, 0i32));
        visited.insert((0i32, 0i32));

        let mut cells = 0;
        for &(limit, key) in &qs {
            while let Some((Reverse(v), x, y)) = q.pop() {
                if v >= limit {
                    q.push((Reverse(v), x, y));
                    break;
                }
                cells += 1;
                for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                        continue;
                    }
                    if visited.contains(&(nx, ny)) {
                        continue;
                    }
                    q.push((Reverse(grid[nx as usize][ny as usize]), nx, ny));
                    visited.insert((nx, ny));
                }
            }
            ans[key] = cells;
        }

        ans
    }
}
