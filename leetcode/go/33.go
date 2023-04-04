package leetcode

/*
    33. Search in Rotated Sorted Array
    ----------------------------------

*/

func search1(nums []int, target int) int {

    // if the array wasnt rotated
    if nums[0] < nums[len(nums) - 1] {
        return binarySearch(nums, target)
    }

    // find the pivot location
    s, e := 0, len(nums) - 1
    for {
        if s >= e {
            break
        }
        middle := (s + e) / 2
        if nums[middle] > nums[s] {
            s = middle
        } else {
            e = middle
        }
    }

    // split nums into 2 slices based on pivot location
    first := nums[0:s+1]
    second := nums[s+1:]

    // check if the target is in the first slice
    // if we find it in the second slice, add the length of the first slice to result
    option1 := binarySearch(first, target)
    if option1 != -1 {
        return option1
    } else {
        option2 := binarySearch(second, target)
        if option2 != -1 {
            return len(first) + option2
        }
    }
    return -1
}

func binarySearch(xs []int, target int) int {
    start := 0
    end := len(xs) - 1
    k := -1

    for {
        if start > end {
            break
        }
        middle := (start + end)/2
        if xs[middle] == target {
            k = middle
            break
        }
        if xs[middle] > target {
            end = middle - 1
        }
        if xs[middle] < target {
            start = middle + 1
        }
    }
    return k
}
