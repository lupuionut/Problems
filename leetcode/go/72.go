package leetcode

func MinDistance(word1 string, word2 string) int {

	dp := make([][]int, len(word1) + 1)

	for k := range(dp) {
		dp[k] = make([]int, len(word2) + 1)
	}

	for i := 0; i <= len(word1); i++ {
		dp[i][0] = i
	}
	for j := 1; j <= len(word2); j++ {
		dp[0][j] = j
	}

	for i := 1; i <= len(word1); i++ {
		for j := 1; j <= len(word2); j++ {
			diff := 0
			if word1[i-1] != word2[j-1] {
				diff = 1
			}
			dp[i][j] = min3(dp[i-1][j-1] + diff, dp[i-1][j] + 1, dp[i][j-1] + 1)
		}
	}

	return dp[len(word1)][len(word2)]
}

func min3(a, b, c int) int {
	min := a
	if b < min {
		min = b
	}
	if c < min {
		min = c
	}
	return min
}
