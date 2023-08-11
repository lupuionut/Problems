// 1861. Rotating the Box
// ----------------------

impl Solution {
    pub fn rotate_the_box(dox: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = dox.len();
        let n = dox[0].len();
        let mut ans = vec![vec![' ' as char; m]; n];

        for i in 0..m {
            let mut stopper = n;
            for j in (0..n).rev() {
                if dox[i][j] == '*' {
                    stopper = j;
                    ans[j][m - i - 1] = '*';
                    continue;
                }
                if dox[i][j] == '.' {
                    ans[j][m - i - 1] = '.';
                    continue;
                }
                if (j + 1) != stopper {
                    ans[j][m - i - 1] = '.';
                }
                ans[stopper - 1][m - i - 1] = '#';
                stopper -= 1;
            }
        }
        ans
    }
}
