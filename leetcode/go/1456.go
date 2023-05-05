package leetcode

/*
    1456. Maximum Number of Vowels in a Substring of Given Length
    -------------------------------------------------------------
    Use prefix sum to store total number of vowels until an index.
    Iterate from k to len(s) and find the max.
*/

func maxVowels(s string, k int) int {
    ps := make([]int, len(s) + 1)
    ps[0] = 0
    max := 0

    isVowel := func(b byte) bool {
        if b == 'a' || b == 'e' || b == 'i' || b == 'o' || b == 'u' {
            return true
        }
        return false
    }

    for i := 1; i <= len(s); i++ {
        v := 0
        if isVowel(s[i-1]) {
            v = 1
        }
        ps[i] = ps[i-1] + v
    }

    for i := k; i <= len(s); i++ {
        current := ps[i] - ps[i-k]
        if current > max {
            max = current
        }
    }
    return max
}
