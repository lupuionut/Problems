/*
    1091. Shortest Path in Binary Matrix
    ------------------------------------
*/

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        fn bfs(grid: &mut Vec<Vec<i32>>) -> i32 {
            let mut ans = 10000;
            let n = grid.len() - 1;
            let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
            queue.push_back((0, 0, 0));

            while queue.len() > 0 {
                let (x, y, distance) = queue.pop_front().unwrap();
                if x < 0 || x > n as i32 || y < 0 || y > n as i32 {
                    continue;
                }
                if grid[x as usize][y as usize] == 1 {
                    continue;
                }
                grid[x as usize][y as usize] = 1;
                if x == n as i32 && y == n as i32 {
                    if (distance + 1) < ans {
                        ans = distance + 1;
                    }
                } else {
                    queue.push_back((x, y - 1, distance + 1));
                    queue.push_back((x, y + 1, distance + 1));
                    queue.push_back((x - 1, y - 1, distance + 1));
                    queue.push_back((x + 1, y, distance + 1));
                    queue.push_back((x - 1, y, distance + 1));
                    queue.push_back((x - 1, y + 1, distance + 1));
                    queue.push_back((x + 1, y - 1, distance + 1));
                    queue.push_back((x + 1, y + 1, distance + 1));
                }
            }
            ans
        }

        let mut grid = grid;
        let res = bfs(&mut grid);
        if res == 10000 {
            return -1;
        }
        res
    }
}
