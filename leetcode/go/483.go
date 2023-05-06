package leetcode

/*
    438. Find All Anagrams in a String
    ----------------------------------
    Make a map with chars that we search
    and their frequency.
    Iterate over s with a sliding window with
    a size lp.
*/

func findAnagrams(s string, p string) []int {
    lp := len(p) - 1
    n := len(s)
    search := make(map[byte]int)
    current := make([]int, 26)
    res := []int{}

    for k := range p {
        search[p[k]] += 1
    }

    for i := 0; i < n; i++ {
        // add new byte to sliding window
        b := int(s[i]) - 97
        current[b] += 1
        if i >= lp {
            anagram := true
            for k := range search {
                t := int(k) - 97
                if current[t] != search[k] {
                    anagram = false
                }
            }
            if anagram == true {
                res = append(res, i - lp)
            }

            // remove the previos byte from window
            // to make sure we keep a fix window
            j := int(s[i - lp]) - 97
            current[j] -= 1
        }
    }
    return res
}
