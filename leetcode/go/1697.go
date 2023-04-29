package leetcode

/*
    1697. Checking Existence of Edge Length Limited Paths
    -----------------------------------------------------
    Using union find to incrementaly check for edges that
    satisfy the query condition.
*/

import "sort"

type SSlice [][]int

func (s SSlice) Len() int {
    return len(s)
}

func (s SSlice) Less(i, j int) bool {
    return s[i][2] < s[j][2]
}

func (s SSlice) Swap(i, j int) {
    t := s[i]
    s[i] = s[j]
    s[j] = t
}

func distanceLimitedPathsExist(n int, edgeList [][]int, queries [][]int) []bool {
    cqueries := make([][]int, len(queries))
    ans := make([]bool, len(queries))

    parent := make([]int, n)
    for i := 0; i < n; i++ {
        parent[i] = i
    }

    var find func(x int) int
    find = func(x int) int {
        if x != parent[x] {
            parent[x] = find(parent[x])
        }
        return parent[x]
    }

    union := func(a, b int) {
        pa := find(a)
        pb := find(b)

        if pa != pb {
            parent[pa] = pb
        }
    }

    for i := 0; i < len(queries); i++ {
        cqueries[i] = make([]int, 4)
        cqueries[i][0] = queries[i][0]
        cqueries[i][1] = queries[i][1]
        cqueries[i][2] = queries[i][2]
        cqueries[i][3] = i
    }

    sort.Sort(SSlice(edgeList))
    sort.Sort(SSlice(cqueries))

    j := 0
    for i := 0; i < len(queries); i++ {
        for j < len(edgeList) && edgeList[j][2] < cqueries[i][2] {
            union(edgeList[j][0], edgeList[j][1])
            j += 1
        }
        if find(cqueries[i][0]) == find(cqueries[i][1]) {
            ans[cqueries[i][3]] = true
        } else {
            ans[cqueries[i][3]] = false
        }
    }

    return ans
}
