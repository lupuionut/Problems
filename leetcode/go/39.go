package leetcode

import "sort"

/*
    39. Combination Sum
    -------------------
    Sort the candidates to take check only for the ones smaller than target.
    For each candidate, we have 2 options, include it each time, or include the
    next one.
*/

func combinationSum(candidates []int, target int) [][]int {
    sort.Ints(candidates)
    possibles := []int{}

    for k := range candidates {
        if candidates[k] <= target {
            possibles = append(possibles, candidates[k])
        }
    }
    sums := new([][]int)

    for k := range possibles {
        path := []int{possibles[k]}
        combine(possibles, k, possibles[k], target, path, sums)
    }
    return *sums
}

func combine(possibles []int, k, sum, target int, path []int, sums *[][]int) {
    if sum == target {
        *sums = append(*sums, path)
        return
    }
    if sum > target {
        return
    }
    path0 := append([]int{possibles[k]}, path...)
    combine(possibles, k, sum + possibles[k], target, path0, sums)
    if k < len(possibles) - 1 {
        path1 := append([]int{}, path...)
        combine(possibles, k+1, sum, target, path1, sums)
    }
}
