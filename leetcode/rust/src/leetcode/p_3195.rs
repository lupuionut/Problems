// 3195. Find the Minimum Area to Cover All Ones I
// -----------------------------------------------

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        let mut up = i32::MAX;
        let mut down = i32::MIN;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    left = left.min(j as i32);
                    right = right.max(j as i32);
                    up = up.min(i as i32);
                    down = down.max(i as i32);
                }
            }
        }

        (right - left + 1) * (down - up + 1)
    }
}
