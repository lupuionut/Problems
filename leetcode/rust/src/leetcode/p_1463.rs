// 1463. Cherry Pickup II
// ----------------------

use std::collections::HashMap;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        fn visit(
            r1row: i32,
            r1col: i32,
            r2row: i32,
            r2col: i32,
            grid: &Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            cache: &mut HashMap<(i32, i32, i32, i32), i32>,
        ) -> i32 {
            let mut current = 0;
            if r1row == r2row && r1col == r2col {
                if visited[r1row as usize][r1col as usize] == false {
                    current += grid[r1row as usize][r1col as usize];
                }
            } else {
                if visited[r1row as usize][r1col as usize] == false {
                    current += grid[r1row as usize][r1col as usize];
                }
                if visited[r2row as usize][r2col as usize] == false {
                    current += grid[r2row as usize][r2col as usize];
                }
            }

            if let Some(&val) = cache.get(&(r1row, r1col, r2row, r2col)) {
                return val;
            }

            visited[r1row as usize][r1col as usize] = true;
            visited[r2row as usize][r2col as usize] = true;
            let new_position_r1 = [
                (r1row + 1, r1col - 1),
                (r1row + 1, r1col),
                (r1row + 1, r1col + 1),
            ];
            let new_position_r2 = [
                (r2row + 1, r2col - 1),
                (r2row + 1, r2col),
                (r2row + 1, r2col + 1),
            ];

            let mut best = 0;
            for (row, col) in new_position_r1 {
                if row >= (grid.len() as i32) || col < 0 || col >= (grid[0].len() as i32) {
                    best = best.max(current);
                    continue;
                }
                for (row1, col1) in new_position_r2 {
                    if row1 >= (grid.len() as i32) || col1 < 0 || col1 >= (grid[0].len() as i32) {
                        best = best.max(current);
                    } else {
                        best =
                            best.max(current + visit(row, col, row1, col1, grid, visited, cache));
                    }
                }
            }

            visited[r1row as usize][r1col as usize] = false;
            visited[r2row as usize][r2col as usize] = false;

            cache.insert((r1row, r1col, r2row, r2col), best);
            best
        }
        let mut cache = HashMap::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        visit(
            0,
            0,
            0,
            grid[0].len() as i32 - 1,
            &grid,
            &mut visited,
            &mut cache,
        )
    }
}
