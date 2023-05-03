package leetcode

/*
    2215. Find the Difference of Two Arrays
    ---------------------------------------
*/

func findDifference(nums1 []int, nums2 []int) [][]int {
    unique1 := make([]int, 2001)
    unique2 := make([]int, 2001)
    res := make([][]int, 2)

    for k := range nums1 {
        n := nums1[k] + 1000
        unique1[n] = 1
    }
    for k := range nums2 {
        n := nums2[k] + 1000
        unique2[n] = 1
    }

    for k := range unique1 {
        n1 := unique1[k]
        n2 := unique2[k]
        if n1 ^ n2 == 1 {
            if n1 == 1 {
                res[0] = append(res[0], k - 1000)
            } else {
                res[1] = append(res[1], k - 1000)
            }
        }
    }

    return res
}
