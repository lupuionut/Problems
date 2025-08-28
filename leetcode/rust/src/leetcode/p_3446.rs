// 3446. Sort Matrix by Diagonals
// ------------------------------

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        if n == 1 {
            return grid;
        }
        let mut ans = vec![vec![0; n]; n];

        for col in 1..n {
            let mut t = vec![];
            let mut d = col;
            let mut r = 0;
            let mut i = 0;
            while d < n {
                t.push(grid[r][d]);
                d += 1;
                r += 1;
            }
            t.sort();
            d = col;
            r = 0;
            while d < n {
                ans[r][d] = t[i];
                d += 1;
                r += 1;
                i += 1;
            }
        }

        for row in 0..n {
            let mut t = vec![];
            let mut d = row;
            let mut c = 0;
            while d < n {
                t.push(grid[d][c]);
                d += 1;
                c += 1;
            }
            t.sort();
            d = row;
            c = 0;
            let mut i = t.len() - 1;
            while d < n {
                ans[d][c] = t[i];
                d += 1;
                c += 1;
                i -= 1;
            }
        }

        ans
    }
}
