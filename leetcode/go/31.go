package leetcode

/*
    31. Next Permutation
    --------------------
    Iterate from right to left and if we find a value that is smaller than the
    previous one, it means we need to make a change, if not, just sort asc the
    slice.

    To make the change, we need to keep track of all visited indexes, so that we
    can find the replacement. The replacement must be the smallest value greater
    than the one we want to replace.
    for ex: 3 6 and we visited values [5,4,7] we must change 3 with 4 since 4 is
    the smallest value greater than 3.
*/

func nextPermutation(nums []int) {
    l := len(nums) - 1
    possibles := []int{}

    findReplacement := func(xs []int, min, max int) int {
        replacement := max
        for k := range xs {
            if nums[xs[k]] > nums[min] && nums[xs[k]] < nums[replacement] {
                replacement = xs[k]
            }
        }
        return replacement
    }

    sort := func(from, to int) {
        for i := from; i < to; i++ {
            for j := from; j < to; j++ {
                if nums[j] > nums[j+1] {
                    t := nums[j+1]
                    nums[j+1] = nums[j]
                    nums[j] = t
                }
            }
        }
    }

    for l > 0 {
        if nums[l-1] < nums[l] {
            replacement := findReplacement(possibles, l-1, l)
            t := nums[l-1]
            nums[l-1] = nums[replacement]
            nums[replacement] = t
            sort(l, len(nums)-1)
            break
        } else {
            possibles = append(possibles, l)
            l -= 1
        }
    }

    if l == 0 {
        sort(0, len(nums) - 1)
    }
}
