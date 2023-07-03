// 1252. Cells with Odd Values in a Matrix
// ---------------------------------------

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];
        let mut odds = 0;

        for i in 0..indices.len() {
            let cell = &indices[i];
            rows[cell[0] as usize] += 1;
            cols[cell[1] as usize] += 1;
        }

        for i in 0..m as usize {
            for j in 0..n as usize {
                let v = rows[i] + cols[j];
                odds += (v % 2);
            }
        }

        odds
    }
}
