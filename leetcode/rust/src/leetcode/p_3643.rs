// 3643. Flip Square Submatrix Vertically
// --------------------------------------
impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = grid.clone();
        let k = k as usize;
        let x = x as usize;
        let y = y as usize;
        let mid = k / 2;

        for i in 0..mid {
            let start = x + i;
            let end = x + k - i - 1;
            for col in y..y + k {
                let t = ans[start][col];
                ans[start][col] = ans[end][col];
                ans[end][col] = t;
            }
        }
        ans
    }
}
