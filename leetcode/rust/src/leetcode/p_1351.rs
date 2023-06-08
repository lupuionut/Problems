// 1351. Count Negative Numbers in a Sorted Matrix
// -----------------------------------------------

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut counter = 0;

        'outer: for i in 0..m {
            for j in 0..n {
                if grid[i][j] < 0 {
                    counter += (n - j) as i32;
                    continue 'outer;
                }
            }
        }

        counter
    }
}
