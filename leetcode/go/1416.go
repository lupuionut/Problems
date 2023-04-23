package leetcode

/*
    1416. Restore The Array
    -----------------------
*/

func numberOfArrays(s string, k int) int {
    N := len(s)
    mod := int(1e9) + 7
    cache := make([]int, len(s))
    for i := range cache {
        cache[i] = -1
    }

    var traverse func(int) int
    traverse = func(i int) int {
        if i == N {
            return 1
        }
        if s[i] == '0' {
            return 0
        }

        if cache[i] != -1 {
            return cache[i]
        }

        count := 0
        idx := i
        val := 0

        for idx < N && val <= k {
            val = val * 10 + int(s[idx] - 48)
            if val <= k {
                count += (traverse(idx + 1) % mod)
            }
            idx += 1
        }
        cache[i] = count % mod
        return cache[i]
    }
    return traverse(0)
}
