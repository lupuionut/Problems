package leetcode

import "math"

/*
    1283. Find the Smallest Divisor Given a Threshold
    -------------------------------------------------
*/

func smallestDivisor(nums []int, threshold int) int {
    l := 1
    r := int(math.Pow10(6))

    canpick := func(n int) bool {
        sum := 0
        for i := 0; i < len(nums); i++ {
            sum += int(math.Ceil(float64(nums[i])/float64(n)))
            if sum > threshold {
                return false
            }
        }

        return true
    }

    for l < r {
        mid := l + (r - l)/2

        if canpick(mid) {
            r = mid
        } else {
            l = mid + 1
        }
    }

    return l
}
