package leetcode

/*
    66. Plus One
    ------------
*/

func plusOne(digits []int) []int {
    carry := 1
    t, res := []int{}, []int{}

    for i := len(digits) - 1; i >= 0; i-- {
        s := digits[i] + carry
        carry = s/10
        t = append(t, s%10)
    }
    if carry > 0 {
        res = []int{carry}
    }

    for i := len(t) - 1; i >= 0; i-- {
        res = append(res, t[i])
    }
    return res
}
