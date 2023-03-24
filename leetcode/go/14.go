package leetcode

func longestCommonPrefix(strs []string) string {
    return longestCommonPrefixRec(strs, "")
}

func longestCommonPrefixRec(ss []string, acc string) string {
    if len(ss[0]) == 0 {
        return acc
    }
    c := ss[0][0]

    for i := 0; i < len(ss); i++ {
        if len(ss[i]) == 0 || ss[i][0] != c {
            return acc
        }
        ss[i] = ss[i][1:]
    }

    return longestCommonPrefixRec(ss, acc + string(c))
}
