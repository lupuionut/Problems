package leetcode

/*
    1170. Compare Strings by Frequency of the Smallest Character
    ------------------------------------------------------------
    Compute the frequencies for words and store inside wordsf.
    Sort it so we can use it for binary search.
    For each query, find the query freq and bin search inside wordsf
    for the index corresponding to the first value greater than
    the query frequency.
*/

import "sort"

func numSmallerByFrequency(queries []string, words []string) []int {
    wordsf := []int{}
    res := []int{}

    f := func(s string) int {
        freq := make(map[byte]int)
        smaller := byte('z')
        for k := range s {
            if s[k] < smaller {
                smaller = s[k]
            }
            freq[s[k]] += 1
        }
        return freq[smaller]
    }

    for k := range words {
        wordsf = append(wordsf, f(words[k]))
    }
    sort.Ints(wordsf)

    binsearch := func(l, r, s int) int {
        for l <= r {
            mid := (l + r) / 2
            if wordsf[mid] < s {
                l = mid + 1
            } else {
                r = mid - 1
            }
        }
        return l
    }

    l := 0
    r := len(wordsf) - 1

    for k := range queries {
        freq := f(queries[k])
        idx := binsearch(l, r, freq + 1)
        res = append(res, r + 1 - idx)
    }

    return res
}
