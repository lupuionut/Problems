// 2373. Largest Local Values in a Matrix
// --------------------------------------

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]; grid.len() - 2];
        for x in 0..grid.len() - 2 {
            for y in 0..grid[0].len() - 2 {
                let mut best = 0;
                for i in x..x + 3 {
                    for j in y..y + 3 {
                        best = best.max(grid[i][j]);
                    }
                }
                ans[x].push(best);
            }
        }
        ans
    }
}
