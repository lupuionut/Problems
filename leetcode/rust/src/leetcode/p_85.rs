// 85. Maximal Rectangle
// ---------------------

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut sum = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut ans = 0;

        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if col == 0 {
                    sum[row][col] = if matrix[row][col] == '0' { 0 } else { 1 };
                } else {
                    if matrix[row][col] != '0' {
                        sum[row][col] = sum[row][col - 1] + 1;
                    }
                }
            }
        }

        for row in 0..sum.len() {
            for col in 0..sum[0].len() {
                let mut i = row + 1;
                let mut min = sum[row][col];
                let mut count = 1;
                ans = ans.max(count * min);
                while i < sum.len() {
                    if sum[i][col] == 0 {
                        break;
                    }
                    if sum[i][col] < min {
                        min = sum[i][col];
                    }
                    count += 1;
                    i += 1;
                    ans = ans.max(count * min);
                }
            }
        }

        ans
    }
}
