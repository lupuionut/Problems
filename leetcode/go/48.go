package leetcode

/*
    48. Rotate Image
    ----------------
    Use a temp array to keep track of replaced values for each
    row, when we do a rotate.
*/

func rotate(matrix [][]int)  {
    n := len(matrix)
    temp := make([][]int, n)
    for k := range temp {
        temp[k] = []int{}
    }

    for i := 0; i < n; i++ {
        temp[i] = matrix[i][i:]
        column := []int{}
        for j := n - 1; j >= 0; j-- {
            column = append(column, matrix[j][i])
        }
        matrix[i] = column
        for j := 0; j <= i; j++ {
            matrix[i][n-j-1] = temp[j][0]
            temp[j] = temp[j][1:]
        }
    }
}
