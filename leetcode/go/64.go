package leetcode

func minPathSum(grid [][]int) int {
    m := len(grid)
    n := len(grid[0])

    dp := make([][]int, m)
    for k := range dp {
        dp[k] = make([]int, n)
        for j := range dp[k] {
            dp[k][j] = -1
        }
    }
    return minPathSumRec(0, 0, m - 1, n - 1, grid, dp)
}

func minPathSumRec(i, j, m, n int, grid, dp [][]int) int {
    min := func(a, b int) int {
        if a < b {
            return  a
        }
        return b
    }

    if dp[i][j] != -1 {
        return dp[i][j]
    }
    //  last column and last row
    if i == m && j == n {
        dp[i][j] = grid[m][n]
        return dp[i][j]
    }
    // last column
    if j == n {
        dp[i][j] = grid[i][j] + minPathSumRec(i+1, j, m, n, grid, dp)
        return dp[i][j]
    }
    // last row
    if i == m {
        dp[i][j] = grid[i][j] + minPathSumRec(i, j+1, m, n, grid, dp)
        return dp[i][j]
    }
    // otherwise
    dp[i][j] = grid[i][j] + min(minPathSumRec(i+1, j, m, n, grid, dp),  minPathSumRec(i, j+1, m, n, grid, dp))
    return dp[i][j]
}
