// 542. 01 Matrix
// --------------

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![-1; mat[0].len()]; mat.len()];
        let mut queue = VecDeque::new();
        let directions = &[(0, 1), (0, -1), (-1, 0), (1, 0)];
        let li = mat.len() as i32;
        let lj = mat[0].len() as i32;

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    ans[i][j] = 0;
                    queue.push_back((i as i32, j as i32));
                }
            }
        }

        while queue.len() > 0 {
            let (i, j) = queue.pop_front().unwrap();

            for (di, dj) in directions {
                let ni = i + di;
                let nj = j + dj;
                if ni >= 0 && ni < li && nj >= 0 && nj < lj && ans[ni as usize][nj as usize] == -1 {
                    ans[ni as usize][nj as usize] = ans[i as usize][j as usize] + 1;
                    queue.push_back((ni, nj));
                }
            }
        }

        ans
    }
}
