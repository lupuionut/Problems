// 864. Shortest Path to Get All Keys
// ----------------------------------

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let mut g: Vec<Vec<char>> = vec![];
        let m = grid.len();
        let n = grid[0].len();
        let mut target_key_mask = 0;
        let mut sum_keys = 0;
        let mut start = (0, 0);
        let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize, i32, i32)> = VecDeque::new();

        for i in 0..m {
            let chars = grid[i].chars().collect::<Vec<char>>();
            g.push(chars);
        }

        for i in 0..m {
            for j in 0..n {
                if g[i][j] == '@' {
                    start = (i, j);
                }
                if g[i][j] as i32 >= 97 && g[i][j] as i32 <= 102 {
                    sum_keys += 1;
                }
            }
        }

        target_key_mask = (1 << sum_keys) - 1;
        queue.push_back((start.0, start.1, 0, 0));
        visited.insert((start.0 as i32, start.1 as i32, 0));

        while queue.len() > 0 {
            // i, j, distance, mask
            let current = queue.pop_front().unwrap();
            let mask = current.3;
            let distance = current.2;

            if mask == target_key_mask {
                return distance;
            }

            let directions = &[
                (current.0 as i32 - 1, current.1 as i32),
                (current.0 as i32 + 1, current.1 as i32),
                (current.0 as i32, current.1 as i32 - 1),
                (current.0 as i32, current.1 as i32 + 1),
            ];

            for &(i, j) in directions {
                if i < 0
                    || i >= m as i32
                    || j < 0
                    || j >= n as i32
                    || g[i as usize][j as usize] == '#'
                {
                    continue;
                }

                let location = g[i as usize][j as usize];

                // if it's a key
                if location as i32 >= 97 && location as i32 <= 102 {
                    let new_mask = mask | (1 << ((location as i32) - 97));
                    if visited.contains(&(i, j, new_mask)) == false {
                        visited.insert((i, j, new_mask));
                        queue.push_back((i as usize, j as usize, distance + 1, new_mask));
                    }
                    continue;
                }

                // if it's a lock
                if location as i32 >= 65 && location as i32 <= 70 {
                    if mask & (1 << ((location as i32) - 65)) == 0 {
                        continue;
                    }
                }

                // otherwise add to queue
                if visited.contains(&(i, j, mask)) == false {
                    visited.insert((i, j, mask));
                    queue.push_back((i as usize, j as usize, distance + 1, mask));
                }
            }
        }

        -1
    }
}
