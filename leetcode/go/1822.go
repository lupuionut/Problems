package leetcode

/*
    1822. Sign of the Product of an Array
    -------------------------------------
*/

func arraySign(nums []int) int {
    res := 1
    for k := range nums {
        if nums[k] == 0 {
            res = 0
            break
        }
        if nums[k] < 0 {
            res = -res
        }
    }
    return res
}
