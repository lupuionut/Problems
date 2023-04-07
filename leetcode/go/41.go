package leetcode

/*
    41. First Missing Positive
    --------------------------
    Check if one is present in the list, if it isn't the first missing
    positive will be one.
    Otherwise, any negative number or number greater than string length
    must be set to 1. This way each number is positive and in the 
    next step we must mark as negative numbers.
    In the next step, for each number, mark the index corresponding to 
    it (for ex if number 5 => idx = 4, number 10 => idx = 9) to negative
    value of the index, if it's not negative, meaning that is already 
    set by other number before.
*/

func firstMissingPositive(nums []int) int {
    one := false
    for k := range nums {
        if nums[k] == 1 {
            one = true
        }
        if nums[k] <= 0 || nums[k] > len(nums) {
            nums[k] = 1
        }
    }
    if one == false {
        return 1
    }

    abs := func(x int) int {
        if x < 0 {
            return x * -1
        }
        return x
    }

    for _, num := range nums {
        idx := abs(num) - 1
        if nums[idx] > 0 {
            nums[idx] *= -1
        }
    }

    for k := range nums {
        if nums[k] > 0 {
            return k + 1
        }
    }

    return len(nums) + 1
}
