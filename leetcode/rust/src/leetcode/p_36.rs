// 36. Valid Sudoku
// ----------------

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = [1 << 9; 9];
        let mut col = [1 << 9; 9];
        let mut square = [1 << 9; 9];

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let k = (i / 3) * 3 + j / 3;
                if board[i][j] == '.' {
                    continue;
                }
                let val = board[i][j].to_digit(10).unwrap() as i32 - 1;
                if row[i] >> val & 1 == 1 {
                    return false;
                }
                if col[j] >> val & 1 == 1 {
                    return false;
                }
                if square[k] >> val & 1 == 1 {
                    return false;
                }
                let n = 1 << val;
                row[i] |= n;
                col[j] |= n;
                square[k] |= n;
            }
        }

        true
    }
}
