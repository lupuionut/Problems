package leetcode

/*
    1639. Number of Ways to Form a Target String Given a Dictionary
    ---------------------------------------------------------------
*/

func numWays(words []string, target string) int {
    mod := int(1e9 + 7)
    lw := len(words[0])
    lt := len(target)

    freq := make([][]int, len(words[0]))
    cache := make([][]int, len(words[0]))

    for k := range freq {
        freq[k] = make([]int, 26)
    }

    for k := range cache {
        cache[k] = make([]int, lt)
        for j := range cache[k] {
            cache[k][j] = -1
        }
    }

    for i := range words {
        for j := range words[i] {
            k := int(words[i][j]) - 97
            freq[j][k] += 1
        }
    }

    var count func(int, int) int
    count = func(wi, ti int) int {
        if ti == lt {
            return 1
        }
        if wi == lw {
            return 0
        }

        if cache[wi][ti] != -1 {
            return cache[wi][ti] % mod
        }

        cache[wi][ti] = 0
        t := int(target[ti]) - 97
        cache[wi][ti] += count(wi + 1, ti + 1) * freq[wi][t]
        cache[wi][ti] += count(wi + 1, ti)

        return cache[wi][ti] % mod
    }

    return count(0,0) % mod
}
