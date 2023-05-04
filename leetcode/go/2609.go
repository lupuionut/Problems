package leetcode

/*
    2609. Find the Longest Balanced Substring of a Binary String
    ------------------------------------------------------------
*/

func findTheLongestBalancedSubstring(s string) int {

    max := 0
    ones, zeros := 0, 0
    previous := byte('1')

    for k := range s {
        if previous == '1' && s[k] == '0' {
            if (2 * ones) > max {
                max = 2 * ones
            }
            ones = 0
            zeros = 0
        }
        if s[k] == '0' {
            zeros += 1
        }
        if s[k] == '1' {
            if zeros > 0 {
                ones += 1
                zeros -= 1
            }
        }
        previous = s[k]
    }

    if (2 * ones) > max {
        max = 2 * ones
    }
    return max
}
