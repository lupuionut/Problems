package leetcode

/*
    498. Diagonal Traverse
    ----------------------
*/

func findDiagonalOrder(mat [][]int) []int {
    rows := len(mat)
    cols := len(mat[0])
    max := rows * cols
    res := []int{}

    cur_row := 0
    cur_col := 0
    direction := 1

    for len(res) < max {
        if direction == 1 {
            for cur_row >= 0 && cur_col < cols {
                res = append(res, mat[cur_row][cur_col])
                cur_row -= 1
                cur_col += 1
            }
            if cur_col == cols {
                cur_row += 2
                cur_col -= 1
            } else {
                cur_row += 1
            }
            direction = 0
        } else {
            for cur_row < rows && cur_col >= 0 {
                res = append(res, mat[cur_row][cur_col])
                cur_row += 1
                cur_col -= 1
            }
            if cur_row == rows {
                cur_row -= 1
                cur_col += 2
            } else {
                cur_col += 1
            }
            direction = 1
        }
    }

    return res
}
