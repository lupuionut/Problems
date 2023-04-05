package leetcode

func minimizeArrayValue(nums []int) int {
    ans := 0
    sum := 0

    max := func(a,b int) int {
        if a > b {
            return a
        }
        return b
    }

    ceil := func(a, b int) int {
        if a%b != 0 {
            return (a/b) + 1
        }
        return a/b
    }

    for k := range nums {
        sum += nums[k]
        ans = max(ans, ceil(sum,k+1))
    }

    return ans
}
