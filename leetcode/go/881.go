package leetcode

/*
    881. Boats to Save People
    -------------------------
*/

import "sort"

func numRescueBoats(people []int, limit int) int {
    sort.Ints(people)
    l := 0
    r := len(people) - 1
    count := 0

    for l <= r {
        count += 1
        if people[l] + people[r] <= limit {
            l += 1
        }
        r -= 1
    }

    return count
}
