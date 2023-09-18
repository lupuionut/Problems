// 1337. The K Weakest Rows in a Matrix
// ------------------------------------

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut strength = vec![(0, 0); mat.len()];

        for i in 0..mat.len() {
            let s = mat[i].iter().sum::<i32>();
            strength[i] = (i, s);
        }
        strength.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        strength
            .drain(..k as usize)
            .map(|x| x.0 as i32)
            .collect::<Vec<i32>>()
    }
}
