package leetcode

/*
    36. Valid Sudoku
    ----------------
    Check for square, line, column
*/

func isValidSudoku(board [][]byte) bool {

    for c := 0; c < 9; c++ {
        column := []byte{}
        for k := range board {
            if c%3 == 2 && k%3 == 2 {
                square := []byte{}
                for i := k-2; i <= k; i++ {
                    for j := c-2; j <= c; j++ {
                        square = append(square, board[i][j])
                    }
                }
                if !isValidSudokuArea(square) {
                    return false
                }
            }
            column = append(column, board[k][c])
            if !isValidSudokuArea(board[k]) {
                return false
            }
        }
        if !isValidSudokuArea(column) {
            return false
        }
    }

    return true
}

func isValidSudokuArea(area []byte) bool {

    idx := func(c byte) int {
        return int(c) - 49
    }
    cells := make([]int, 9)

    for i := 0; i < len(area); i++ {
        if area[i] == '.' {
          continue
        }
        k := idx(area[i])
        cells[k] += 1
        if cells[k] > 1 {
            return false
        }
    }
    return true
}
