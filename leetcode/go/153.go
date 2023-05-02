package leetcode

/*
    153. Find Minimum in Rotated Sorted Array
    -----------------------------------------
*/

func findMin(nums []int) int {
    n := len(nums)
    if nums[0] < nums[n-1] {
        return nums[0]
    }

    start, end := 0, n - 1

    for start <= end {
        middle := (start + end)/2
        if nums[middle] >= nums[start] {
            if nums[middle] > nums[end] {
                start = middle + 1
            } else {
                end = middle - 1
            }
        } else {
            if nums[middle] < nums[end] {
                end = middle
            } else {
                start = middle + 1
            }
        }
    }

    return nums[start]
}
