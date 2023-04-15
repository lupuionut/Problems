package leetcode

/*
    2218. Maximum Value of K Coins From Piles
    -----------------------------------------
*/

func maxValueOfCoins(piles [][]int, k int) int {

    cache := make([][]int, len(piles))
    for i := range cache {
        cache[i] = make([]int, k + 1)
        for j := range cache[i] {
            cache[i][j] = -1
        }
    }

    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    var dp func(int, int) int
    dp = func(idx int, coins int) int {
        if idx == len(piles) {
            return 0
        }

        if cache[idx][coins] != -1 {
            return cache[idx][coins]
        }

        maxCoins := coins
        if len(piles[idx]) < maxCoins {
            maxCoins = len(piles[idx])
        }

        // don't pick a coin from this pile
        cache[idx][coins] = dp(idx+1, coins)

        // pick a coin
        pick := 0
        for i := 0; i < maxCoins; i++ {
            pick += piles[idx][i]
            remains := dp(idx+1, coins - i - 1)
            cache[idx][coins] = max(cache[idx][coins], pick + remains)
        }

        return cache[idx][coins]
    }

    return dp(0, k)
}
