package leetcode

/*

    47. Permutations II
    -------------------
*/

import "sort"

func permuteUnique(nums []int) [][]int {
    sort.Ints(nums)

    freq := make(map[int]int)
    for k := range nums {
        freq[nums[k]] += 1
    }

    res := [][]int{}
    t := []int{}

    var dfs func()
    dfs = func() {
        if len(t) == len(nums) {
            temp := append([]int{}, t...)
            res = append(res, temp)
            return
        }

        for k := range freq {
            if freq[k] > 0 {
                t = append(t, k)
                freq[k] -= 1
                dfs()
                freq[k] += 1
                t = t[0: len(t) - 1]
            }
        }
    }

    dfs()

    return res
}
