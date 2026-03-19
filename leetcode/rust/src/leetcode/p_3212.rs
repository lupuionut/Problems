// 3212. Count Submatrices With Equal Frequency of X and Y
// -------------------------------------------------------
impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        let mut prev_acc: Vec<Option<(i32, i32)>> = vec![None; grid[0].len()];
        for i in 0..grid.len() {
            let mut acc = (0, 0);
            for j in 0..grid[i].len() {
                if grid[i][j] == 'X' {
                    acc.0 += 1;
                } else if grid[i][j] == 'Y' {
                    acc.1 += 1;
                }
                if let Some(prev) = prev_acc[j] {
                    let x = acc.0 + prev.0;
                    let y = acc.1 + prev.1;
                    if x > 0 && x == y {
                        ans += 1;
                    }
                    prev_acc[j] = Some((x, y));
                } else {
                    prev_acc[j] = Some(acc);
                    if acc.0 > 0 && acc.0 == acc.1 {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
