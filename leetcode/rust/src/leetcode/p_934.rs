// 934. Shortest Bridge
// find the first 1 part of first island and use dfs to find all 1's for first island
// mark each 1 as -1
// run another loop and for each -1 we encounter run a bfs to find first 1 (second island)
// get the min from this loop

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut shortest = 10000;

        fn dfs(row: usize, col: usize, grid: &mut Vec<Vec<i32>>) -> () {
            if row < 0 || row >= grid.len() {
                return;
            }
            if col < 0 || col >= grid.len() {
                return;
            }
            if grid[row][col] == 1 {
                grid[row][col] = -1;
                dfs(row + 1, col, grid);
                dfs(row - 1, col, grid);
                dfs(row, col + 1, grid);
                dfs(row, col - 1, grid);
            }
        }

        fn bfs(row: usize, col: usize, grid: &Vec<Vec<i32>>) -> i32 {
            let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
            let mut visited: Vec<Vec<i32>> = vec![vec![0; grid.len()]; grid.len()];

            queue.push_back((row, col, 0));
            let mut dist = 0;

            while queue.len() > 0 {
                let (row, col, distance) = queue.pop_front().unwrap();

                if row < 0 || row >= grid.len() {
                    continue;
                }
                if col < 0 || col >= grid.len() {
                    continue;
                }

                if visited[row][col] == 1 {
                    continue;
                }

                visited[row][col] = 1;

                if grid[row][col] == 1 {
                    dist = distance;
                    break;
                } else {
                    queue.push_back((row + 1, col, distance + 1));
                    queue.push_back((row - 1, col, distance + 1));
                    queue.push_back((row, col + 1, distance + 1));
                    queue.push_back((row, col - 1, distance + 1));
                }
            }

            (dist - 1) as i32
        }

        fn min(a: i32, b: i32) -> i32 {
            if a < b {
                return a;
            }
            b
        }

        'outer: for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] == 1 {
                    dfs(i, j, &mut grid);
                    break 'outer;
                }
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] == -1 {
                    shortest = min(shortest, bfs(i, j, &grid));
                }
            }
        }

        shortest
    }
}
