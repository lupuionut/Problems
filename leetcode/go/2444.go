package leetcode
/*
2 3 1 2 3 5 3 2 5 2 3 1 3 7
*/


func CountSubarrays(nums []int, minK int, maxK int) int {
    count := 0
    nextIdx := 0
    min := -1
    max := -1

    maximum := func(a,b int) int {
        if a > b {
            return a
        }
        return b
    }

    minimum := func(a,b int) int {
        if a < b {
            return a
        }
        return b
    }

    for i := 0 ; i < len(nums); i++ {
        if nums[i] < minK || nums[i] > maxK {
            nextIdx = i + 1
        }

        if nums[i] == minK {
            min = i
        }

        if nums[i] == maxK {
            max = i
        }

        count += maximum(0, minimum(min, max) - nextIdx + 1)
    }

    return count
}
