package leetcode

/*
    2554. Maximum Number of Integers to Choose From a Range I
    ---------------------------------------------------------
    Sort the banned slice and start to check from 1 to n what
    number is not in the banned.
*/

import "sort"

func maxCount(banned []int, n int, maxSum int) int {

    sort.Ints(banned)
    sum, counter := 0, 0

    contains := func(n int) bool {
        l, r := 0, len(banned) - 1

        for l <= r {
            mid := (l + r)/2
            if banned[mid] < n {
                l = mid + 1
            } else {
                r = mid - 1
            }
        }

        if l == len(banned) {
            return false
        }
        return banned[l] == n
    }

    for i := 1; i <= n; i++ {
        if !contains(i) {
            sum += i
            if sum > maxSum {
                break
            } else {
                counter += 1
            }
        }
    }

    return counter
}
