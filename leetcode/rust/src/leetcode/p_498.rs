// 498. Diagonal Traverse
// ----------------------

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        fn go(
            mut i: i32,
            mut j: i32,
            up: bool,
            rows: i32,
            cols: i32,
            mat: &Vec<Vec<i32>>,
            ans: &mut Vec<i32>,
        ) {
            loop {
                ans.push(mat[i as usize][j as usize]);
                if up == true {
                    if i - 1 < 0 || j + 1 >= cols {
                        break;
                    }
                    i -= 1;
                    j += 1;
                } else {
                    if i + 1 >= rows || j - 1 < 0 {
                        break;
                    }
                    i += 1;
                    j -= 1;
                }
            }
            if i == rows - 1 && j == cols - 1 {
                return;
            }
            if up == true {
                if j + 1 == cols {
                    go(i + 1, j, false, rows, cols, mat, ans);
                } else {
                    go(i, j + 1, false, rows, cols, mat, ans);
                }
            } else {
                if i + 1 == rows {
                    go(i, j + 1, true, rows, cols, mat, ans);
                } else {
                    go(i + 1, j, true, rows, cols, mat, ans);
                }
            }
        }

        let mut ans = vec![];
        go(
            0,
            0,
            true,
            mat.len() as i32,
            mat[0].len() as i32,
            &mat,
            &mut ans,
        );
        ans
    }
}
