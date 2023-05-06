package leetcode

/*
    1498. Number of Subsequences That Satisfy the Given Sum Condition
    -----------------------------------------------------------------
    Number of subsets between l and r is 2^(r-l)
*/

import "sort"

func numSubseq(nums []int, target int) int {
    sort.Ints(nums)
    n := len(nums)
    l, r := 0, n - 1
    ans := 0
    mod := int(1e9 + 7)
    powers := make([]int, n)

    powers[0] = 1
    for i := 1; i < n; i++ {
        powers[i] = (powers[i-1] * 2) % mod
    }

    for l <= r {
        if nums[l] + nums[r] <= target {
            ans += powers[r-l] % mod
            l += 1
        } else {
            r -= 1
        }
    }

    return ans % mod
}
