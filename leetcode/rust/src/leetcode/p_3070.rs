// 3070. Count Submatrices with Top-Left Element and Sum Less Than k
// -----------------------------------------------------------------
impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;

        let mut acc = vec![0; grid[0].len()];
        let mut ps = grid.clone();
        // do the prefix sum
        for i in 0..grid.len() {
            for j in 1..grid[i].len() {
                ps[i][j] += ps[i][j - 1];
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let total = ps[i][j] + acc[j];
                if total <= k {
                    ans += 1;
                }
                acc[j] = total;
            }
        }

        ans
    }
}
