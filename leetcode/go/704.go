package leetcode

/*
    704. Binary Search
    ------------------
*/

func search(nums []int, target int) int {
    start := 0
    end := len(nums) - 1
    k := -1

    for {
        if start > end {
            break
        }

        middle := (start + end)/2
        if nums[middle] == target {
            k = middle
            break
        }
        if nums[middle] > target {
            end = middle - 1
        }
        if nums[middle] < target {
            start = middle + 1
        }
    }

    return k
}
