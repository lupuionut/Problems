package leetcode

/*
    879. Profitable Schemes
    -----------------------
*/

func profitableSchemes(n int, minProfit int, group []int, profit []int) int {
    mod := int(1e9) + 7
    ret := 0
    cache := make([][][]int, n + 1)
    for k := range cache {
        cache[k] = make([][]int, len(group))
        for j := range cache[k] {
            cache[k][j] = make([]int, minProfit + 1)
            for l := range cache[k][j] {
                cache[k][j][l] = -1
            }
        }
    }

    var count func(people int, i int, p int) int
    count = func(people int, i int, p int) int {
        if people < 0 {
            return 0
        }
        if p < 0 {
            p = 0
        }
        if i == len(group) {
            if p == 0 {
                return 1
            }
            return 0
        }

        if cache[people][i][p] != -1 {
            return cache[people][i][p]
        }

        involved := count(people - group[i], i+1, p - profit[i])
        not_involved := count(people, i+1, p)
        cache[people][i][p] = (involved % mod) + (not_involved % mod)
        return cache[people][i][p]
    }

    ret = count(n, 0, minProfit)
    return ret % mod
}
