package leetcode

/*
    1967. Number of Strings That Appear as Substrings in Word
    ---------------------------------------------------------
*/

func numOfStrings(patterns []string, word string) int {
    ans := 0

    contains := func(pattern string) bool {
        lp := len(pattern)
        for i := 0; i <= len(word) - lp; i++ {
            if word[i: i+lp] == pattern {
                return true
            }
        }
        return false
    }

    for k := range patterns {
        if contains(patterns[k]) {
            ans += 1
        }
    }

    return ans
}
