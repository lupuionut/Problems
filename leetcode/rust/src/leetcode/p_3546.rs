// 3546. Equal Sum Grid Partition I
// --------------------------------
impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut col_sums = vec![0; grid[0].len()];
        let mut row_sums = vec![0; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                col_sums[j] += grid[i][j] as i64;
                row_sums[i] += grid[i][j] as i64;
            }
        }
        let col_sum = col_sums.iter().sum::<i64>();
        let row_sum = row_sums.iter().sum::<i64>();

        let mut t = 0;
        if col_sum % 2 == 0 {
            for i in 0..col_sums.len() {
                t += col_sums[i];
                if t == col_sum / 2 {
                    return true;
                }
            }
        }
        if row_sum % 2 == 0 {
            t = 0;
            for i in 0..row_sums.len() {
                t += row_sums[i];
                if t == row_sum / 2 {
                    return true;
                }
            }
        }

        false
    }
}
