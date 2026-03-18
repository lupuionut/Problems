// 3070. Count Submatrices with Top-Left Element and Sum Less Than k
// -----------------------------------------------------------------
impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;
        let mut t = 0;
        let mut i = 0;
        // first column
        while i < grid.len() {
            t += grid[i][0];
            i += 1;
            if t > k {
                break;
            } else {
                ans += 1;
            }
        }

        let mut acc = vec![0; grid[0].len()];
        let mut ps = grid.clone();
        // do the prefix sum
        for i in 0..grid.len() {
            for j in 1..grid[i].len() {
                ps[i][j] += ps[i][j - 1];
            }
        }

        //println!("{:?}", ps);
        for i in 0..grid.len() {
            for j in 1..grid[i].len() {
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
