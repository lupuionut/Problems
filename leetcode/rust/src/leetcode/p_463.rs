// 463. Island Perimeter
// ---------------------

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    // left
                    if j == 0 || (j > 0 && grid[i][j - 1] == 0) {
                        ans += 1;
                    }
                    // right
                    if j == grid[0].len() - 1 || (j < grid[0].len() && grid[i][j + 1] == 0) {
                        ans += 1;
                    }
                    // top
                    if i == 0 || (i > 0 && grid[i - 1][j] == 0) {
                        ans += 1;
                    }
                    // bottom
                    if i == grid.len() - 1 || (i < grid.len() && grid[i + 1][j] == 0) {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
