package leetcode

/*
    2140. Solving Questions With Brainpower
    ---------------------------------------
*/

func mostPoints(questions [][]int) int64 {
    cache := make([]int, len(questions))
    for k := range cache {
        cache[k] = -1
    }

    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    var dfs func(i int) int
    dfs = func(i int) int {
        if i >= len(questions) {
            return 0
        }
        if cache[i] != -1 {
            return cache[i]
        }
        cache[i] = max(questions[i][0] + dfs(i + 1 + questions[i][1]), dfs(i+1))
        return cache[i]
    }

    return int64(dfs(0))
}
