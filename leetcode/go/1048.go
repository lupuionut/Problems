package leetcode

/*
    1048. Longest String Chain
    --------------------------
*/

func longestStrChain(words []string) int {

    position := make(map[string]int)
    edges := make([][]int, len(words))
    score := make([]int, len(words))

    for k := range edges {
        edges[k] = []int{}
    }

    for k := range words {
        position[words[k]] = k
    }

    for i := 0; i < len(words); i++ {
        s := words[i]
        for j := 0; j < len(s); j++ {
            possible := s[0:j] + s[j+1:]
            if p, ok := position[possible]; ok {
                edges[p] = append(edges[p], i)
            }
        }
    }
    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    var longest func(v int) int
    longest = func(v int) int {
        if score[v] > 0 {
            return score[v]
        }
        score[v] = 1
        for k := range edges[v] {
            score[v] = max(score[v], longest(edges[v][k]) + 1)
        }

        return score[v]
    }

    ans := 0
    for i := 0; i < len(words); i++ {
        ans = max(ans, longest(i))
    }

    return ans
}
