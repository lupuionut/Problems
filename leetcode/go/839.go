package leetcode

/*
    839. Similar String Groups
    --------------------------
*/

func numSimilarGroups(strs []string) int {
    isSimilar := func (s1, s2 string) bool {
        diff := 0
        for k := range s1 {
            if s1[k] != s2[k] {
                diff += 1
                if diff > 2 {
                    return false
                }
            }
        }
        if diff == 0 || diff == 2 {
            return true
        }
        return false
    }

    var dfs func(str string, strs []string, visited map[string]int)
    dfs = func(str string, strs []string, visited map[string]int) {
        if visited[str] == 1 {
            return
        }
        visited[str] = 1
        for k := range strs {
            if isSimilar(strs[k], str) {
                dfs(strs[k], strs, visited)
            }
        }
    }

    counter := 0
    visited := make(map[string]int)

    for k := range strs {
        str := strs[k]
        if visited[str] == 0 {
            dfs(str, strs, visited)
            counter += 1
        }
    }

    return counter
}
