// 1765. Map of Highest Peak
// -------------------------

use std::collections::VecDeque;
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut q = VecDeque::new();
        let m = is_water.len();
        let n = is_water[0].len();
        let mut ans = vec![vec![-1; n]; m];
        let directions = &[(1, 0), (-1, 0), (0, -1), (0, 1)];

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    q.push_back((i as i32, j as i32, 0));
                }
            }
        }
        if q.len() == 0 {
            q.push_back((0, 0, 1));
        }
        while q.len() > 0 {
            if let Some((i, j, v)) = q.pop_front() {
                if ans[i as usize][j as usize] != -1 {
                    continue;
                }
                ans[i as usize][j as usize] = v;
                for &(dx, dy) in directions {
                    let nx = i + dx;
                    let ny = j + dy;
                    if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                        continue;
                    }
                    if ans[nx as usize][ny as usize] != -1 {
                        continue;
                    }
                    q.push_back((nx, ny, v + 1));
                }
            } else {
                break;
            }
        }

        ans
    }
}
