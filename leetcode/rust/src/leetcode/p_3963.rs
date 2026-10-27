// 3963. Create Grid With Exactly One Path
// ---------------------------------------

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut grid = vec![vec!['#'; n as usize]; m as usize];
        for i in 0..n as usize {
            grid[0][i] = '.';
        }
        for i in 0..m as usize {
            grid[i][n as usize - 1] = '.';
        }
        grid.iter().map(|v| v.iter().collect::<String>()).collect()
    }
}
