package leetcode

/*
    1572. Matrix Diagonal Sum
    -------------------------
    > O x x x O      x x x x x      x x x x x
      x x x x x    > x O x O x      x x x x x
      x x x x x      x x x x x    > x x O x x
      x x x x x    > x O x O x      x x x x x
    > O x x x O      x x x x x      x x x x x
*/

func diagonalSum(mat [][]int) int {
    top, bottom := 0, len(mat) - 1
    left, right := 0, len(mat) - 1
    sum := 0

    for top < bottom {
        sum += mat[top][left] + mat[top][right] + mat[bottom][left] + mat[bottom][right]
        top += 1
        bottom -= 1
        left += 1
        right -= 1
    }

    if top == bottom {
        sum += mat[top][left]
    }

    return sum
}
