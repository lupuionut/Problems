// 1886. Determine Whether Matrix Can Be Obtained By Rotation
// ----------------------------------------------------------
impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        fn rotate(src: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut ans = vec![vec![0; src[0].len()]; src.len()];
            let n = src.len();
            for i in 0..n {
                for j in 0..n {
                    ans[i][j] = src[n - j - 1][i];
                }
            }
            ans
        }

        let mut t = mat;
        for i in 0..4 {
            t = rotate(t);
            if t == target {
                return true;
            }
        }
        false
    }
}
