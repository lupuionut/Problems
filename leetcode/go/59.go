package leetcode

/*
    59. Spiral Matrix II
    --------------------
*/

func generateMatrix(n int) [][]int {
    left, right := 0, n - 1
    top, bottom := 0, n - 1
    counter, max := 1, n * n + 1

    matrix := make([][]int, n)
    for k := range matrix {
        matrix[k] = make([]int, n)
    }

    for top <= bottom && left <= right {
        for i := left; i <= right; i++ {
            matrix[top][i] = counter
            counter += 1
        }
        top += 1

        if counter == max {
            break
        }

        for i := top; i <= bottom; i++ {
            matrix[i][right] = counter
            counter += 1
        }
        right -= 1

        if counter == max {
            break
        }

        for i := right; i >= left; i-- {
            matrix[bottom][i] = counter
            counter += 1
        }
        bottom -= 1

        if counter == max {
            break
        }

        for i := bottom; i >= top; i-- {
            matrix[i][left] = counter
            counter += 1
        }
        left += 1

        if counter == max {
            break
        }
    }

    return matrix
}
