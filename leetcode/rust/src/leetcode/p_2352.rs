// 2352. Equal Row and Column Pairs

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut columns: Vec<Vec<i32>> = Vec::new();
        let mut rows: Vec<Vec<i32>> = Vec::new();

        for i in 0..grid.len() {
            let mut row: Vec<i32> = Vec::new();
            let mut column: Vec<i32> = Vec::new();
            for j in 0..grid.len() {
                row.push(grid[i][j]);
                column.push(grid[j][i])
            }
            columns.push(column);
            rows.push(row);
        }

        for row in rows.iter() {
            for column in columns.iter() {
                if row == column {
                    ans += 1;
                }
            }
        }
        ans
    }
}
