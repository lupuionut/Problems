// 2812. Find the Safest Path in a Grid
// ------------------------------------

use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let mut safety = vec![vec![-1; grid.len()]; grid.len()];
        let mut d = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut queue = VecDeque::new();
        let mut heap = BinaryHeap::new();

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] == 1 {
                    safety[i][j] = 0;
                    queue.push_back((i as i32, j as i32, 0));
                }
            }
        }
        while queue.len() > 0 {
            let (row, col, dist) = queue.pop_front().unwrap();
            for (dx, dy) in &d {
                let new_x = row + dx;
                let new_y = col + dy;
                if new_x < 0
                    || new_x >= grid.len() as i32
                    || new_y < 0
                    || new_y >= grid.len() as i32
                    || safety[new_x as usize][new_y as usize] != -1
                {
                    continue;
                }
                safety[new_x as usize][new_y as usize] = dist + 1;
                queue.push_back((new_x, new_y, dist + 1));
            }
        }

        let mut visited = vec![vec![false; grid.len()]; grid.len()];
        heap.push((safety[0][0], 0, 0));

        while heap.len() > 0 {
            let (curr_safety, row, col) = heap.pop().unwrap();
            if row as usize == grid.len() - 1 && col as usize == grid.len() - 1 {
                return curr_safety;
            }
            if visited[row as usize][col as usize] == true {
                continue;
            }
            visited[row as usize][col as usize] = true;
            for (dx, dy) in &d {
                let new_x = row + dx;
                let new_y = col + dy;
                if new_x < 0
                    || new_x >= grid.len() as i32
                    || new_y < 0
                    || new_y >= grid.len() as i32
                    || visited[new_x as usize][new_y as usize] == true
                {
                    continue;
                }
                heap.push((
                    curr_safety.min(safety[new_x as usize][new_y as usize]),
                    new_x,
                    new_y,
                ))
            }
        }

        -1
    }
}
