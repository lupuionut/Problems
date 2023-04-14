package leetcode

import "sort"

/*
    40. Combination Sum II
    ----------------------
*/

func combinationSum2(candidates []int, target int) [][]int {
    sort.Ints(candidates)

    res := [][]int{}
    t := []int{}

    var backt func(int, int, []int, []int)
    backt = func(i, target int, t, candidates []int) {
        if target == 0 {
            temp := append([]int{}, t...)
            res = append(res, temp)
        }
        if target <= 0 {
            return
        }
        for j := i; j < len(candidates); j++ {
            if i != j && candidates[j] == candidates[j-1] {
                continue
            }
            t = append(t, candidates[j])
            backt(j + 1, target - candidates[j], t, candidates)
            t = t[0: len(t) - 1]
        }
    }

    backt(0, target, t, candidates)
    return res
}
