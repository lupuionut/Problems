package leetcode

/*
    1312. Minimum Insertion Steps to Make a String Palindrome
    ---------------------------------------------------------
    LCS = length of common substring between string and reversed
    LS = length of string
    The mininimum insertion value = LS - LCS
*/

func minInsertions(s string) int {

    ls := len(s)
    reversed := []byte{}
    for i := ls - 1; i >= 0; i-- {
        reversed = append(reversed, s[i])
    }
    ss := string(reversed)

    cache := make([][]int, ls)
    for k := range cache {
        cache[k] = make([]int, ls)
        for j := range cache[k] {
            cache[k][j] = -1
        }
    }

    lcs := common_substring_length(s, ss, 0, 0, cache)
    return ls - lcs
}

func common_substring_length(a, b string, i, j int, cache [][]int) int {
    if len(a) == 0 || len(b) == 0 {
        return 0
    }
    if cache[i][j] != -1 {
        return cache[i][j]
    }
    if a[0] == b[0] {
        cache[i][j] = 1 + common_substring_length(a[1:], b[1:], i+1, j+1, cache)
        return cache[i][j]
    }
    advancea := common_substring_length(a[1:], b, i+1, j, cache)
    advanceb := common_substring_length(a, b[1:], i, j+1, cache)
    if advancea > advanceb {
        cache[i][j] = advancea
    } else {
        cache[i][j] = advanceb
    }
    return cache[i][j]
}
