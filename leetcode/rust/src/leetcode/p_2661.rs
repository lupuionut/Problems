// 2661. First Completely Painted Row or Column
// --------------------------------------------

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let r = mat.len();
        let c = mat[0].len();
        let mut rows = vec![0; r];
        let mut cols = vec![0; c];
        let mut position = vec![(0, 0); arr.len() + 1];

        for i in 0..r {
            for j in 0..c {
                let k = mat[i][j] as usize;
                position[k] = (i, j);
            }
        }

        for i in 0..arr.len() {
            let (row, col) = position[arr[i] as usize];
            rows[row] += 1;
            cols[col] += 1;
            if rows[row] == c || cols[col] == r {
                return i as i32;
            }
        }

        -1
    }
}
