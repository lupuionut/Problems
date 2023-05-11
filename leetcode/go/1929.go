package leetcode

/*
    1929. Concatenation of Array
    ----------------------------
*/

func getConcatenation(nums []int) []int {
    nums = append(nums, nums...)
    return nums
}
