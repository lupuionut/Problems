package leetcode

/*
    91. Decode Ways
    ---------------
    For each step, we have 2 options to take 1 or 2 char.
    So dp[k] = dfs(k+1) + dfs(k+2)
*/

func numDecodings(s string) int {

    dp := make([]int, len(s))
    for k := range dp {
        dp[k] = -1
    }

    var dfs func(k int) int
    dfs = func(k int) int {
        if k >= len(s) {
            return 1
        }
        if s[k] == '0' {
            return 0
        }
        if dp[k] != -1 {
            return dp[k]
        }
        dp[k] = 0
        dp[k] = dfs(k+1)

        if k+1 >= len(s) {
            return dp[k]
        }
        if s[k] != '1' && s[k] != '2' {
            return dp[k]
        }
        if s[k] == '2' && (s[k+1] == '7' || s[k+1] == '8' || s[k+1] == '9') {
            return dp[k]
        }

        dp[k] += dfs(k+2)
        return dp[k]
    }

    return dfs(0)
}
