// 2946. Matrix Similarity After Cyclic Shifts
// -------------------------------------------
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let c = mat[0].len();
        let r = mat.len();
        let mut k = k as usize;
        for i in 0..r {
            if k > c {
                k %= c;
            }
            for j in 0..c {
                let z = (j + k) % c;
                if mat[i][z] != mat[i][j] {
                    return false;
                }
            }
        }
        true
    }
}
