// 1391. Check if There is a Valid Path in a Grid
// ----------------------------------------------
use std::collections::HashMap;
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let mut allowed: HashMap<(i32, i32, i32), Vec<i32>> = HashMap::new();
        allowed.insert((1, 0, -1), vec![1, 4, 6]);
        allowed.insert((1, 0, 1), vec![1, 3, 5]);
        allowed.insert((2, -1, 0), vec![2, 3, 4]);
        allowed.insert((2, 1, 0), vec![2, 5, 6]);
        allowed.insert((3, 0, -1), vec![1, 4, 6]);
        allowed.insert((3, 1, 0), vec![2, 5, 6]);
        allowed.insert((4, 0, 1), vec![1, 3, 5]);
        allowed.insert((4, 1, 0), vec![2, 5, 6]);
        allowed.insert((5, 0, -1), vec![1, 4, 6]);
        allowed.insert((5, -1, 0), vec![2, 3, 4]);
        allowed.insert((6, 0, 1), vec![1, 3, 5]);
        allowed.insert((6, -1, 0), vec![2, 3, 4]);
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        fn dfs(
            i: usize,
            j: usize,
            grid: &Vec<Vec<i32>>,
            allowed: &HashMap<(i32, i32, i32), Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if i == grid.len() - 1 && j == grid[0].len() - 1 {
                return true;
            }
            visited[i][j] = true;
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut ans = false;
            for &(dx, dy) in &dirs {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;
                if ni < 0 || ni >= grid.len() as i32 || nj < 0 || nj >= grid[0].len() as i32 {
                    continue;
                }
                if visited[ni as usize][nj as usize] == true {
                    continue;
                }

                let mut ok = false;
                if let Some(allows) = allowed.get(&(grid[i][j], dx, dy)) {
                    for &allow in allows {
                        if grid[ni as usize][nj as usize] == allow {
                            ok = true;
                            break;
                        }
                    }
                }
                if !ok {
                    continue;
                }
                ans = ans || dfs(ni as usize, nj as usize, grid, allowed, visited);
            }
            ans
        }
        dfs(0, 0, &grid, &allowed, &mut visited)
    }
}
