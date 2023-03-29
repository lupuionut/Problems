package leetcode

import "sort"

/*
    1402. Reducing Dishes

    To maximize the satisfaction, sort the input in ascending order so that
    the time will also increase, since LIKE := satisfaction[i] * time[i]

    For each time, we have 2 options: either we use that satisfaction and increase the time
    or we don't use that satisfaction but the time will stay the same, so we
    must choose the max of these 2 options.
*/

func MaxSatisfaction(satisfaction []int) int {

    sort.Ints(satisfaction)
    cache := make([][]int, len(satisfaction) + 1)
    for k := range cache {
        cache[k] = make([]int, len(satisfaction) + 1)
        for j := range cache[k] {
            cache[k][j] = -1
        }
    }

    return MaxSatisfactionRec(satisfaction, cache, 1, 1)
}

func MaxSatisfactionRec(satisfaction []int, cache [][]int, step, time int) int {
    if len(satisfaction) == 1 {
        if satisfaction[0] < 0 {
            return 0
        } else {
            return time * satisfaction[0]
        }
    }

    if cache[step][time] != -1 {
        return cache[step][time]
    }

    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    cache[step][time] = max(
        (satisfaction[0] * time) + MaxSatisfactionRec(satisfaction[1:], cache, step+1, time+1),
        0 + MaxSatisfactionRec(satisfaction[1:], cache, step+1, time),
    )

    return cache[step][time]
}

