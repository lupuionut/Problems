package leetcode

/*
    646. Maximum Length of Pair Chain
    ---------------------------------
    Sort in the order of ending. Iterate over
    the intervals and each time we find an
    interval that doesn't satisfy the condition
    we keep the previous interval.
*/

import "sort"

type Slice646 [][]int

func (s Slice646) Len() int {
    return len(s)
}

func (s Slice646) Less(i, j int) bool {
    return s[i][1] < s[j][1]
}

func (s Slice646) Swap(i, j int) {
    t := s[i]
    s[i] = s[j]
    s[j] = t
}

func findLongestChain(pairs [][]int) int {
    sort.Sort(Slice646(pairs))

    current := pairs[0]
    count := 1

    for i := 1; i < len(pairs); i++ {
        previous := current
        current = pairs[i]

        if current[0] > previous[1] && current[1] > previous[1] {
            count += 1
        } else {
            current = previous
        }
    }

    return count
}
