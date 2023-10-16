// 119. Pascal's Triangle II
// -------------------------

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }

        let mut row = vec![1, 1];
        let mut idx = 1;

        while idx < row_index {
            let mut new_row = vec![1];
            let mut iter = row.windows(2);
            while let Some(x) = iter.next() {
                new_row.push(x[0] + x[1]);
            }
            new_row.push(1);
            row = new_row;
            idx += 1;
        }

        row
    }
}
