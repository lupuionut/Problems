package leetcode

// 1444. Number of Ways of Cutting a Pizza

func ways(pizza []string, k int) int {

    mod := 1000000007
    m, n := len(pizza), len(pizza[0])
    cache := make([][][]int, m)

    for i := range cache {
        cache[i] = make([][]int, n)
        for j := range cache[i] {
            cache[i][j] = make([]int, k)
            for h := range cache[i][j] {
                cache[i][j][h] = -1
            }
        }
    }

    apples := make([][]int, m + 1)
    for i := range apples {
        apples[i] = make([]int, n + 1)
    }

    for i, row := range pizza {
        for j, value := range row {
            apples[i+1][j+1] = apples[i+1][j] + apples[i][j+1] - apples[i][j]
            if value == 'A' {
                apples[i+1][j+1]++
            }
        }
    }

    var recurse func(int, int, int) int

    recurse = func(i, j, k int) int {
        if cache[i][j][k] != -1 {
            return cache[i][j][k]
        }

        if k == 0 {
            if apples[m][n] - apples[m][j] - apples[i][n] + apples[i][j] > 0 {
                return 1
            }
            return 0
        }

        res := 0
        for x := i + 1; x < m; x++ {
            if apples[x][n] - apples[x][j] - apples[i][n] + apples[i][j] > 0 {
                res = (res + recurse(x, j, k - 1)) % mod
            }
        }
        for y := j + 1; y < n; y++ {
            if apples[m][y] - apples[m][j] - apples[i][y] + apples[i][j] > 0 {
                res = (res + recurse(i, y, k - 1)) % mod
            }
        }
        cache[i][j][k] = res
        return res
    }
    return recurse(0, 0, k-1)
}
