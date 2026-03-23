// 1594. Maximum Non Negative Product in a Matrix
// ----------------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let mut q = vec![];
        let mut best = i64::MIN;
        q.push(((0, 0), grid[0][0] as i64));
        let mut seen = HashSet::new();
        while q.len() > 0 {
            if let Some((pos, val)) = q.pop() {
                if pos.0 == grid.len() - 1 && pos.1 == grid[0].len() - 1 {
                    best = best.max(val);
                    continue;
                }
                if pos.0 < grid.len() - 1 {
                    let dx = pos.0 + 1;
                    let dy = pos.1;
                    let v = val * grid[dx][dy] as i64;
                    if !seen.contains(&(dx, dy, v)) {
                        seen.insert((dx, dy, v));
                        q.push(((dx, dy), v));
                    }
                }
                if pos.1 < grid[0].len() - 1 {
                    let dx = pos.0;
                    let dy = pos.1 + 1;
                    let v = val * grid[dx][dy] as i64;
                    if !seen.contains(&(dx, dy, v)) {
                        seen.insert((dx, dy, v));
                        q.push(((dx, dy), v));
                    }
                }
            }
        }
        if best < 0 {
            return -1;
        }
        (best % 1_000_000_007) as i32
    }
}
