// 861. Score After Flipping Matrix
// --------------------------------

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut best = vec![0; cols];
        let mut ans = 0;

        for i in 0..rows {
            if grid[i][0] == 0 {
                for j in 0..cols {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }

        for j in 0..cols {
            let mut count = (0, 0);
            for i in 0..rows {
                if grid[i][j] == 0 {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            }
            best[j] = count.0.max(count.1);
        }

        for i in 0..cols {
            ans *= 2;
            ans += best[i];
        }

        ans
    }
}
