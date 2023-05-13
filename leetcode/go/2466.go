package leetcode

/*
    2466. Count Ways To Build Good Strings
    --------------------------------------
*/

func countGoodStrings(low int, high int, zero int, one int) int {
    mod := 1000000007
    cache := make([]int, high + 1)
    for k := range cache {
        cache[k] = -1
    }

    var dfs func(t int) int
    dfs = func(t int) int {
        if t > high {
            return 0
        }
        if cache[t] != -1 {
            return cache[t]
        }
        if t >= low {
            cache[t] = (1 + dfs(t + zero) + dfs(t + one)) % mod
        } else {
            cache[t] = (dfs(t + zero) + dfs(t + one)) % mod
        }
        return cache[t]
    }

    return dfs(0)
}
