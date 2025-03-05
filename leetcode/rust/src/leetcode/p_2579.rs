// 2579. Count Total Number of Colored Cells
// -----------------------------------------

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let mut n = n as i64;
        let mut sum = 2 * n - 1;
        n -= 1;
        while n > 0 {
            sum += (4 * n - 2);
            n -= 1;
        }
        sum
    }
}
