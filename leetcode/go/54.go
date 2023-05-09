package leetcode

/*
    54. Spiral Matrix
    -----------------
    Traverse top (left -> right) and increase the top.
    Traverse right (top -> down) and decrease right.
    Traverse bottom (left -> right) and decrease bottom.
    Travese left (bottom -> up) and increase left.
*/

func spiralOrder(matrix [][]int) []int {
    left, right := 0, len(matrix[0]) - 1
    top, bottom := 0, len(matrix) - 1
    max := len(matrix) * len(matrix[0])

    ans := []int{}

    for top <= bottom && left <= right {
        for i := left; i <= right; i++ {
            ans = append(ans, matrix[top][i])
        }
        top += 1

        if len(ans) == max {
            break
        }

        for i := top; i <= bottom; i++ {
            ans = append(ans, matrix[i][right])
        }
        right -= 1

        if len(ans) == max {
            break
        }

        for i := right; i >= left; i-- {
            ans = append(ans, matrix[bottom][i])
        }
        bottom -= 1

        if len(ans) == max {
            break
        }

        for i := bottom; i >= top; i-- {
            ans = append(ans, matrix[i][left])
        }
        left += 1

        if len(ans) == max {
            break
        }
    }

    return ans
}
