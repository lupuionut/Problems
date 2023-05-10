package leetcode

/*
    1347. Minimum Number of Steps to Make Two Strings Anagram
    ---------------------------------------------------------
    Find how many chars are in common.
    Length of string - common = number of steps.
*/

func minSteps(s string, t string) int {
    first := make([]int, 26)
    second := make([]int, 26)

    for k := range s {
        a := int(s[k]) - 97
        b := int(t[k]) - 97
        first[a] += 1
        second[b] += 1
    }

    common := 0
    for i := 0; i < 26; i++ {
        if first[i] > 0 && second[i] > 0 {
            if first[i] < second[i] {
                common += first[i]
            } else {
                common += second[i]
            }
        }
    }

    return len(s) - common
}
