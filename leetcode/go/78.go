package leetcode

/*
    78. Subsets
    -----------
*/

func subsets(nums []int) [][]int {
    res := [][]int{}

    subset := []int{}
    var combine func(int)
    combine = func(n int) {
        if n == len(nums) {
            res = append(res, append([]int{}, subset...))
            return
        }

        subset = append(subset, nums[n])
        combine(n+1)

        subset = subset[0:len(subset) - 1]
        combine(n+1)
    }
    combine(0)
    return res
}
