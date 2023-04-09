package leetcode

/*
    1857. Largest Color Value in a Directed Graph
    ---------------------------------------------
*/

func largestPathValue(colors string, edges [][]int) int {

    adj := make(map[int][]int)
    visited := make(map[int]int)
    path := make(map[int]int)
    inf := len(colors) + 1
    dp := make([][]int, len(colors))

    for k := range dp {
        dp[k] = make([]int, 26)
        for c := range dp[k] {
            dp[k][c] = 0
        }
    }

    res := 0

    for k := range edges {
        adj[edges[k][0]] = append(adj[edges[k][0]], edges[k][1])
    }

    toInt := func(c byte) int {
        return (97 - int(c)) * -1
    }

    max := func (a,b int) int {
        if a > b {
            return a
        }
        return b
    }

    var dfs func(int) int
    dfs = func(node int) int {
        if path[node] == 1 {
            return inf
        }
        if visited[node] == 1 {
            return 0
        }
        visited[node] = 1
        path[node] = 1
        key := toInt(colors[node])
        dp[node][key] = 1
        neighbors := adj[node]
        current_max := 1
        for _, neighbor := range neighbors {
            if dfs(neighbor) == inf {
                return inf
            }
            for c := 0; c < 26; c++ {
                add := 0
                if key == c {
                    add = 1
                }
                dp[node][c] = max(dp[node][c], dp[neighbor][c] + add)
                if dp[node][c] > current_max {
                    current_max = dp[node][c]
                }
            }
        }
        path[node] = 0
        return current_max
    }

    for i := 0; i < len(colors); i++ {
        res = max(res, dfs(i))
    }

    if res == inf {
        return -1
    }
    return res
}
