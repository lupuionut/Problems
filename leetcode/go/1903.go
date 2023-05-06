package leetcode

/*
    1903. Largest Odd Number in String
    ----------------------------------
*/

func largestOddNumber(num string) string {
    n := len(num)
    for i := n-1; i >= 0; i-- {
        c := int(num[i]) - 48
        if c & 1 == 1 {
            return num[0:i+1]
        }
    }
    return ""
}
