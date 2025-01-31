// 827. Making A Large Island
// --------------------------

use std::collections::HashMap;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut groups = vec![];
        let mut position_in_group = HashMap::new();
        let mut zeros = vec![];
        let mut best = 0;

        fn visit(
            i: i32,
            j: i32,
            grid: &Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            group: usize,
            position_in_group: &mut HashMap<(usize, usize), usize>,
        ) -> i32 {
            if visited[i as usize][j as usize] == true {
                return 0;
            }
            position_in_group.insert((i as usize, j as usize), group);
            visited[i as usize][j as usize] = true;
            let mut ans = 1;
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            for (dx, dy) in &directions {
                let nx = dx + i;
                let ny = dy + j;

                if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid.len() as i32 {
                    continue;
                }
                if visited[nx as usize][ny as usize] == true || grid[nx as usize][ny as usize] == 0
                {
                    continue;
                }
                ans += visit(nx, ny, grid, visited, group, position_in_group);
            }
            ans
        }

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    zeros.push((i, j));
                    continue;
                }
                let k = groups.len();
                if grid[i][j] == 1 && visited[i][j] == false {
                    let t = visit(
                        i as i32,
                        j as i32,
                        &grid,
                        &mut visited,
                        k,
                        &mut position_in_group,
                    );
                    groups.push(t);
                    best = best.max(t);
                }
            }
        }

        if zeros.len() == 0 {
            return best;
        }

        for i in 0..zeros.len() {
            let mut t = 0;
            let x = zeros[i].0 as i32;
            let y = zeros[i].1 as i32;
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            let mut seen = vec![];
            for (dx, dy) in &directions {
                let nx = dx + x;
                let ny = dy + y;
                let mut count = true;
                if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid.len() as i32 {
                    continue;
                }
                if grid[nx as usize][ny as usize] == 0 {
                    continue;
                }
                if let Some(&p) = position_in_group.get(&(nx as usize, ny as usize)) {
                    for i in 0..seen.len() {
                        if seen[i] == p {
                            count = false;
                        }
                    }
                    if count {
                        seen.push(p);
                        t += groups[p];
                    }
                }
            }
            best = best.max(1 + t);
        }

        best
    }
}
