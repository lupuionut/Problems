// 118. Pascal's Triangle
// ----------------------

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut i = 1;
        let mut ans = vec![vec![]; n];
        ans[0] = vec![1];

        while i < n {
            let mut temp = vec![1];
            let mut j = 1;
            let m = ans[i - 1].len();
            while j < m {
                temp.push(ans[i - 1][j - 1] + ans[i - 1][j]);
                j += 1;
            }
            temp.push(1);
            ans[i] = temp;
            i += 1;
        }

        ans
    }
}
