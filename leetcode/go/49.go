package leetcode

/*
    49. Group Anagrams
    ------------------
    Use sorted version of a string as a key for a map.
    For each string we encounter, we sort it and check if the map contains that
    key, if so we inserted into the map.
    At the end we iterate over the map and create the final array.
*/

import "sort"

func groupAnagrams(strs []string) [][]string {
    m := make(map[string][]string)
    a := [][]string{}

    for k := range strs {
        var key word
        key = []byte(strs[k])
        sort.Sort(key)
        mk := string(key)
        if val, ok := m[mk]; ok {
            m[mk] = append(val, strs[k])
        } else {
            m[mk] = []string{strs[k]}
        }
    }

    for k := range m {
        a = append(a, m[k])
    }

    return a
}

type word []byte

func (w word) Len() int {
    return len(w)
}

func (w word) Less(i, j int) bool {
    return w[i] < w[j]
}

func (w word) Swap(i, j int) {
    temp := w[i]
    w[i] = w[j]
    w[j] = temp
}
