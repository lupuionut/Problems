package leetcode

// 16. 3Sum Closest

import (
    "sort"
)

func ThreeSumClosest(nums []int, target int) int {

    sort.Ints(nums)
    init := false
    best := 0
    n := len(nums)

    delta := func(a, b int) int {
        c := a - b
        if c < 0 {
            return -1 * c
        }
        return c
    }

    for i := 0; i < n - 2; i++ {
        a := nums[i]
        start := i+1
        end := n-1

        for {
            if start >= end {
                break
            }
            b := nums[start]
            c := nums[end]
            sum := a + b + c
            if init == false || (delta(target, sum) < delta(target, best)) {
                best = sum
                init = true
            }
            if sum == target {
                break
            } else {
                if sum > target {
                    end -= 1
                } else {
                    start += 1
                }
            }
        }
    }

    return best
}
