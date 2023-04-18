package leetcode

/*
    1768. Merge Strings Alternately
    -------------------------------
*/

func mergeAlternately(word1 string, word2 string) string {
    if len(word1) == 0 && len(word2) == 0 {
        return ""
    }
    if len(word1) == 0 {
        return word2
    }
    if len(word2) == 0 {
        return word1
    }
    return string(word1[0]) + string(word2[0]) + mergeAlternately(word1[1:], word2[1:])
}
