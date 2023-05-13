package leetcode

import "sort"

/*
    2008. Maximum Earnings From Taxi
    --------------------------------
    For each ride, we have 2 options:
    - we take that ride, and continue from where end of ride
    - we skip and go to next ride
*/

type Slice_2008 [][]int

func (s Slice_2008) Len() int {
    return len(s)
}

func (s Slice_2008) Less(i, j int) bool {
    if s[i][0] == s[j][0] {
        return s[i][1] < s[j][1]
    }
    return s[i][0] < s[j][0]
}

func (s Slice_2008) Swap(i, j int) {
    t := s[i]
    s[i] = s[j]
    s[j] = t
}

func maxTaxiEarnings(n int, rides [][]int) int64 {

    cache := make([]int, len(rides))
    for k := range cache {
        cache[k] = -1
    }

    sort.Sort(Slice_2008(rides))

    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    binsearch := func(xs [][]int, x int) int {
        l := 0
        r := len(xs) - 1

        for l <= r {
            mid := (l + r) / 2
            if xs[mid][0] < x {
                l = mid + 1
            } else {
                r = mid - 1
            }
        }
        return l
    }

    var dfs func(idx int) int
    dfs = func(idx int) int {
        if idx >= len(rides) {
            return 0
        }
        if cache[idx] != -1 {
            return cache[idx]
        }

        ride := rides[idx]
        dollars := ride[1] - ride[0] + ride[2]
        nidx := binsearch(rides, ride[1])
        cache[idx] = max(dollars + dfs(nidx), dfs(idx+1))
        return cache[idx]
    }

    return int64(dfs(0))
}
