package leetcode

/*
    1046. Last Stone Weight
    -----------------------
*/

import "sort"

func lastStoneWeight(stones []int) int {
    for len(stones) > 1 {
        sort.SliceStable(stones, func(i, j int) bool {
            return stones[i] > stones[j]
        })
        d := stones[0] - stones[1]
        if d != 0 {
            stones = append(stones, d)
        }
        stones = stones[2:]
    }
    if len(stones) == 0 {
        return 0
    }

    return stones[0]
}
