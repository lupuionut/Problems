package leetcode

/*
    53. Maximum Subarray
    --------------------
    Using Kadane's algo, see here:
    https://www.youtube.com/watch?v=umt7t1_X8Rc
*/

func maxSubArray(nums []int) int {

    ans, a := int(-1e4), 0
    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    for k := range nums {
        a += nums[k]
        ans = max(ans, a)
        a = max(a, 0)
    }

    return ans
}
