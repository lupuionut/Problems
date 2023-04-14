package leetcode

/*
    516. Longest Palindromic Subsequence
    ------------------------------------
*/

func longestPalindromeSubseq(s string) int {
    n := len(s)
    cache := make([][]int, n)
    for k := range cache {
        cache[k] = make([]int, n)
        for j := range cache[k] {
            cache[k][j] = -1
        }
    }

    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    var getLongest func(start, end int) int

    getLongest = func(start, end int) int {
        if start > end {
            return 0
        }
        if start == end {
            return 1
        }

        if cache[start][end] != -1 {
            return cache[start][end]
        }

        if s[start] == s[end] {
            cache[start][end] = 2 + getLongest(start + 1, end - 1)
        } else {
            cache[start][end] = max(getLongest(start+1, end), getLongest(start, end-1))
        }
        return cache[start][end]
    }

    return getLongest(0, n-1)
}


/*
func longestPalindromeSubseq(s string) int {
    reversed := []byte{}
    for i := len(s) - 1; i >= 0; i-- {
        reversed = append(reversed, s[i])
    }
    ss := string(reversed)
    // build the cache
    cache := make([][]string, len(s))
    for k := range cache {
        cache[k] = make([]string, len(s))
        for j := range cache[k] {
            cache[k][j] = "-"
        }
    }

    common := choice(s, ss, 0, 0, cache)
    return len(common)
}

func choice(a, b string, i, j int, cache [][]string) string {
    if len(a) == 0 {
        return ""
    }
    if len(b) == 0 {
        return ""
    }
    if cache[i][j] != "-" {
        return cache[i][j]
    }
    if a[0] == b[0] {
        cache[i][j] = string(a[0]) + choice(a[1:], b[1:], i+1, j+1, cache)
        return cache[i][j]
    }

    take := choice(a[1:], b, i+1, j, cache)
    nottake := choice(a, b[1:], i, j+1, cache)

    if len(take) > len(nottake) {
        cache[i][j] = take
    } else {
        cache[i][j] = nottake
    }
    return cache[i][j]
}
*/
